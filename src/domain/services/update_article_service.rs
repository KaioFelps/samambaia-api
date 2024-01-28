use std::error::Error;
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::bad_request_error::BadRequestError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};
use crate::util::{RolePermissions, verify_role_has_permission};

pub struct UpdateArticleParams {
    pub user_id: Uuid,
    pub article_id: Uuid,
    pub cover_url: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub approved: Option<bool>,
}
pub struct UpdateArticleService<
ArticleRepository: ArticleRepositoryTrait,
UserRepository: UserRepositoryTrait
> {
    user_repository: Box<UserRepository>,
    article_repository: Box<ArticleRepository>,
}

impl
<ArticleRepository: ArticleRepositoryTrait,
UserRepository: UserRepositoryTrait>
UpdateArticleService<ArticleRepository, UserRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>, user_repository: Box<UserRepository>) -> Self {
        UpdateArticleService {
            article_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: UpdateArticleParams) -> Result<Article, Box<dyn Error>> {
        // checks if there is something to be updated

        if params.cover_url.is_none() && params.title.is_none() && params.cover_url.is_none() && params.approved.is_none() {
            return Err(Box::new(BadRequestError::new()));
        }

        // checks user exists

        let user_on_db = &self.user_repository.find_by_id(&params.user_id).await;

        if user_on_db.is_err() { return Err(Box::new(InternalError::new())); }

        let user_on_db = user_on_db.as_ref().unwrap().to_owned();

        if user_on_db.is_none() { return Err(Box::new(UnauthorizedError::new())) }

        // article verifications

        let article_on_db = &self.article_repository.find_by_id(params.article_id).await;

        if article_on_db.is_err() { return Err(Box::new(InternalError::new())); }
        
        let article_on_db = article_on_db.as_ref().unwrap();

        if article_on_db.is_none() { return Err(Box::new(ResourceNotFoundError::new())) }

        let mut article = article_on_db.clone().unwrap();

        // checks user is allowed to perform the update
        let user_can_update = verify_role_has_permission(
            &user_on_db.as_ref().unwrap().role().unwrap().clone().to_owned(),
            RolePermissions::UpdateArticle
        );

        let user_can_approve = verify_role_has_permission(
            &user_on_db.as_ref().unwrap().role().unwrap().clone().to_owned(),
            RolePermissions::ApproveArticle
        );

        let user_can_disapprove = verify_role_has_permission(
            &user_on_db.as_ref().unwrap().role().unwrap().clone().to_owned(),
            RolePermissions::DisapproveArticle
        );

        if !user_can_approve && params.approved.is_some() { return Err(Box::new(UnauthorizedError::new())); }
        if !user_can_disapprove && params.approved.is_some() && params.approved.unwrap() == false { return Err(Box::new(UnauthorizedError::new())); }
        
        let user_is_author = article.author_id() == user_on_db.unwrap().id();

        if !user_can_update && !user_is_author { return Err(Box::new(UnauthorizedError::new())); }

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

        let response = &self.article_repository.save(article).await;

        if response.as_ref().is_ok() {
            return Ok(response.as_ref().unwrap().clone());
        }

        else {
           return Err(Box::new(InternalError::new()));
        }
    }
}


#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::errors::unauthorized_error::UnauthorizedError;
    use crate::{domain::repositories::user_repository::MockUserRepositoryTrait, errors::resource_not_found::ResourceNotFoundError};
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;
    use super::{Article, UpdateArticleParams};

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        let mut article_db: Vec<Article> = vec![];

        // mocking user repo
        mocked_user_repo
        .expect_find_by_id()
        .returning(|id| {
            let fake_user = User::new_from_existing(
                id.clone().to_owned(),
                "Fake name".to_string(),
                "password".to_string(),
                chrono::Utc::now().naive_utc(),
                None,
                Some(Role::Writter)
            );

            Ok(Some(fake_user))
        });

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
            user_repository: Box::new(mocked_user_repo),
            article_repository: Box::new(mocked_article_repo)
        };

        let result = service.exec(UpdateArticleParams {
            user_id: article.author_id(),
            article_id: article.id(),
            approved: Some(true),
            title: None,
            content: None,
            cover_url: None,
        }).await;

        assert!(result.unwrap_err().is::<UnauthorizedError>()); // writter can't approve any article

        let result = service.exec(UpdateArticleParams {
            user_id: article.author_id(),
            article_id: article.id(),
            approved: None,
            title: Some("Título atualizado".to_string()),
            content: Some("Conteúdo atualizado".to_string()),
            cover_url: None,
        }).await;

        assert_eq!("Título atualizado", result.unwrap().title());
    }
}
