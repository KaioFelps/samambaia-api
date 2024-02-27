use std::error::Error;
use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::errors::bad_request_error::BadRequestError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};
use crate::util::{RolePermissions, verify_role_has_permission};

use crate::{LOG_SEP, R_EOL};

pub struct UpdateArticleParams {
    pub user_id: Uuid,
    pub user_role: Role,
    pub article_id: Uuid,
    pub cover_url: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub approved: Option<bool>,
}
pub struct UpdateArticleService<ArticleRepository: ArticleRepositoryTrait> {
    article_repository: Box<ArticleRepository>,
}

impl
<ArticleRepository: ArticleRepositoryTrait> UpdateArticleService<ArticleRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>) -> Self {
        UpdateArticleService {
            article_repository,
        }
    }

    pub async fn exec(&self, params: UpdateArticleParams) -> Result<Article, Box<dyn Error>> {
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

        // if user is autho but does no longer belong to the team, he can't delete his own article either.
        if user_is_author && params.user_role == Role::User { return Err(Box::new(UnauthorizedError::new())); }

        // article modifying

        if !params.content.is_none() {
            article.set_content(params.content.unwrap())
        }

        if !params.title.is_none() {
            article.set_title(params.title.unwrap());
        }

        if !params.cover_url.is_none() {
            article.set_cover_url(params.cover_url.unwrap());
        }

        if !params.approved.is_none() {
            article.set_approved(params.approved.unwrap());
        }

        let response = self.article_repository.save(article).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Update Article Service, while saving the article on the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }
        
        Ok(response.unwrap())
    }
}


#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::errors::unauthorized_error::UnauthorizedError;
    use crate::errors::resource_not_found::ResourceNotFoundError;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::role::Role;
    use super::{Article, UpdateArticleParams};

    #[tokio::test]
    async fn test() {
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        let mut article_db: Vec<Article> = vec![];

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string()
        );

        article_db.push(article.clone());

        // mocking article repo
        let db = article_db.clone().to_owned();
        mocked_article_repo
        .expect_find_by_id()
        .returning(move |id| {
            let db = &db;

            let mut index: Option<usize> = None;

            for (i, item) in db.iter().enumerate() {
                if item.id() == id {
                    index = Some(i);
                    break;
                }
            }

            if index.is_none() {
                return Ok(None);
            }

            let index = index.unwrap();

            Ok(Some(db[index].clone()))
        });

        mocked_article_repo
        .expect_save()
        .returning(move |param_article: Article| {
            let article_id = param_article.id().clone();

            article_db.push(param_article.clone());

            let mut index: Option<usize> = None;

            for (i, item) in article_db.iter().enumerate() {
                if item.id() == article_id {
                    index = Some(i);
                    break;
                }
            }

            if index.is_none() {
                return Err(Box::new(ResourceNotFoundError::new()));
            }

            let index = index.unwrap();

            article_db[index] = param_article;

            Ok(article_db[0].clone())
        });

        let service = super::UpdateArticleService {
            article_repository: Box::new(mocked_article_repo)
        };

        let result = service.exec(UpdateArticleParams {
            user_id: article.author_id(),
            user_role: Role::Writter,
            article_id: article.id(),
            approved: Some(true),
            title: None,
            content: None,
            cover_url: None,
        }).await;

        assert!(result.unwrap_err().is::<UnauthorizedError>()); // writter can't approve any article

        let result = service.exec(UpdateArticleParams {
            user_id: article.author_id(),
            user_role: Role::Writter,
            article_id: article.id(),
            approved: None,
            title: Some("Título atualizado".to_string()),
            content: Some("Conteúdo atualizado".to_string()),
            cover_url: None,
        }).await;

        assert_eq!("Título atualizado", result.unwrap().title());
    }
}
