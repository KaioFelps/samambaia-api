use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::errors::bad_request_error::BadRequestError;
use crate::errors::error::DomainErrorTrait;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};
use crate::util::{generate_service_internal_error, RolePermissions, verify_role_has_permission};

use crate::{LOG_SEP, R_EOL};
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;

pub struct UpdateArticleParams {
    pub user_id: Uuid,
    pub user_role: Role,
    pub article_id: Uuid,
    pub cover_url: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub approved: Option<bool>,
    pub author_id: Option<Uuid>,
    pub tag_id: Option<i32>
}
pub struct UpdateArticleService<ArticleRepository: ArticleRepositoryTrait, ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_repository: Box<ArticleRepository>,
    article_tag_repository: Box<ArticleTagRepository>
}

impl
<ArticleRepository: ArticleRepositoryTrait, ArticleTagRepository: ArticleTagRepositoryTrait>
UpdateArticleService<ArticleRepository, ArticleTagRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>, article_tag_repository: Box<ArticleTagRepository>) -> Self {
        UpdateArticleService {
            article_repository,
            article_tag_repository
        }
    }

    pub async fn exec(&self, params: UpdateArticleParams) -> Result<Article, Box<dyn DomainErrorTrait>> {
        // checks if there is something to be updated

        if params.cover_url.is_none() && params.title.is_none() && params.cover_url.is_none() && params.approved.is_none() {
            return Err(Box::new(BadRequestError::new()));
        }

        // article verifications

        let article_on_db = self.article_repository.find_by_id(params.article_id).await;

        if article_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Update Article Service, while finding article by id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                article_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }
        
        let article_on_db = article_on_db.as_ref().unwrap();

        if article_on_db.is_none() { return Err(Box::new(ResourceNotFoundError::new())) }

        let mut article = article_on_db.clone().unwrap();

        // checks user is allowed to perform the update
        let user_can_update = verify_role_has_permission(
            &params.user_role,
            RolePermissions::UpdateArticle
        );

        let user_can_approve = verify_role_has_permission(
            &params.user_role,
            RolePermissions::ApproveArticle
        );

        let user_can_disapprove = verify_role_has_permission(
            &params.user_role,
            RolePermissions::DisapproveArticle
        );

        if !user_can_approve && params.approved.is_some() { return Err(Box::new(UnauthorizedError::new())); }
        if !user_can_disapprove && params.approved.is_some() && params.approved.unwrap() == false { return Err(Box::new(UnauthorizedError::new())); }
        
        let user_is_author = article.author_id() == params.user_id;

        if !user_can_update && !user_is_author { return Err(Box::new(UnauthorizedError::new())); }

        // if user is author but does no longer belong to the team, he can't delete his own article either.
        if user_is_author && params.user_role == Role::User { return Err(Box::new(UnauthorizedError::new())); }

        let user_can_change_article_author = verify_role_has_permission(
            &params.user_role,
            RolePermissions::ChangeArticleAuthor
        );

        if !user_can_change_article_author && params.author_id.is_some() { return Err(Box::new(UnauthorizedError::new())) }

        // modifies the article where requested
        if params.author_id.is_some() {
            article.set_author_id(params.author_id.unwrap())
        }

        if params.content.is_some() {
            article.set_content(params.content.unwrap())
        }

        if params.title.is_some() {
            article.set_title(params.title.unwrap());
        }

        if params.cover_url.is_some() {
            article.set_cover_url(params.cover_url.unwrap());
        }

        if params.approved.is_some() {
            article.set_approved(params.approved.unwrap());
        }

        if params.tag_id.is_some() {
            let tag = match self.get_tag_by_id(params.tag_id.unwrap()).await {
                Ok(tag) => tag,
                Err(error) => return Err(error),
            };

            article.set_tag_id(tag.id());
            article.set_tag_value(tag.value().into());
        }

        let response = self.article_repository.save(article).await;

        if response.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred in Update Article Service, while saving the article on the database".into(),
                &response.unwrap_err()
            ));
        }

        Ok(response.unwrap())
    }

    async fn get_tag_by_id(&self, tag_id: i32) -> Result<ArticleTag, Box<dyn DomainErrorTrait>> {
        let tag = self.article_tag_repository.find_by_id(tag_id).await;
        if tag.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred in Update Article Service, while finding article tag by id".into(),
                &tag.unwrap_err()
            ));
        }

        let tag = tag.unwrap();
        if tag.is_none() {
            return Err(Box::new(BadRequestError::new_with_message(format!("Tag with id '{}' not found.", tag_id))))
        }

        Ok(tag.unwrap())
    }
}


#[cfg(test)]
mod test {
    use http::StatusCode;
    use uuid::Uuid;
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::errors::resource_not_found::ResourceNotFoundError;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::article_tag_repository::MockArticleTagRepositoryTrait;
    use super::{Article, UpdateArticleParams};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();
        let mut mocked_article_tag_repo: MockArticleTagRepositoryTrait = MockArticleTagRepositoryTrait::new();

        let mut article_db: Arc<Mutex<Vec<Article>>> = Arc::new(Mutex::new(vec![]));
        let mut tag_db: Arc<Mutex<Vec<ArticleTag>>> = Arc::new(Mutex::new(vec![]));

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string(),
            1,
            "Foo".into()
        );

        let article_tag = ArticleTag::new_from_existing(2, "Bar".into());

        tag_db.lock().unwrap().push(article_tag);
        article_db.lock().push(article.clone());

        // mocking article repo
        let db = Arc::clone(&article_db);
        mocked_article_repo
            .expect_find_by_id()
            .returning(move |id| {
                let mut article: Option<Article> = None;

                for item in db.iter() {
                    if item.id() == id {
                        article = Some(item.clone());
                        break;
                    }
                }

                Ok(article)
            });

        let db = Arc::clone(&mut article_db);
        mocked_article_repo
            .expect_save()
            .returning(move |param_article: Article| {
                for (i, item) in article_db.iter().enumerate() {
                    if item.id() == param_article.id() {
                        db.lock()[i] = param_article.clone();
                        return Ok(param_article);
                    }
                }

                return Err(Box::new(ResourceNotFoundError::new()));
            });

        // mocking article tag repo
        let db = Arc::clone(&tag_db);
        mocked_article_tag_repo
            .expect_find_by_id()
            .returning(move |tag_id| {
                let mut tag = None;

                for _tag in db.lock().iter() {
                    if _tag.id().eq(tag_id) {
                        tag = Some(_tag.clone());
                    }
                }

                return Ok(tag)
            });

        let service = super::UpdateArticleService {
            article_repository: Box::new(mocked_article_repo),
            article_tag_repository: Box::new(mocked_article_tag_repo)
        };

        let result = service.exec(UpdateArticleParams {
            user_id: article.author_id(),
            user_role: Role::Writter,
            article_id: article.id(),
            approved: Some(true),
            title: None,
            content: None,
            cover_url: None,
            author_id: None,
            tag_id: None
        }).await;

        assert_eq!(result.unwrap_err().code(), &StatusCode::UNAUTHORIZED); // writer can't approve any article

        let result = service.exec(UpdateArticleParams {
            user_id: article.author_id(),
            user_role: Role::Writter,
            article_id: article.id(),
            approved: None,
            title: Some("Título atualizado".to_string()),
            content: Some("Conteúdo atualizado".to_string()),
            cover_url: None,
            author_id: None,
            tag_id: Some(2)
        }).await;

        let result = result.unwrap();

        assert_eq!("Título atualizado", result.title());
        assert_eq!("Bar", result.tag_value());
    }
}
