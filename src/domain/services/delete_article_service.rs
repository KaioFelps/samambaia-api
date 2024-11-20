use log::error;
use uuid::Uuid;

use crate::domain::repositories::article_comment_repository::ArticleCommentRepositoryTrait;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};
use crate::util::{verify_role_has_permission, RolePermissions};

use crate::{LOG_SEP, R_EOL};

pub struct DeleteArticleParams {
    pub user_id: Uuid,
    pub article_id: Uuid,
}
pub struct DeleteArticleService<
    AR: ArticleRepositoryTrait,
    ACR: ArticleCommentRepositoryTrait,
    UR: UserRepositoryTrait,
> {
    article_repository: Box<AR>,
    article_comment_repository: Box<ACR>,
    user_repository: Box<UR>,
}

impl<AR: ArticleRepositoryTrait, ACR: ArticleCommentRepositoryTrait, UR: UserRepositoryTrait>
    DeleteArticleService<AR, ACR, UR>
{
    pub fn new(
        article_repository: Box<AR>,
        article_comment_repository: Box<ACR>,
        user_repository: Box<UR>,
    ) -> Self {
        DeleteArticleService {
            article_repository,
            article_comment_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: DeleteArticleParams) -> Result<(), Box<dyn DomainErrorTrait>> {
        let user_on_db = &self.user_repository.find_by_id(&params.user_id).await;

        if user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Article Service, while finding user by Id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let user_on_db = user_on_db.as_ref().unwrap().to_owned();

        if user_on_db.is_none() {
            return Err(Box::new(UnauthorizedError::new()));
        }

        // article verifications

        let article_on_db = &self.article_repository.find_by_id(params.article_id).await;

        if article_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Article Service, while finding article by Id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                article_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let article_on_db = article_on_db.as_ref().unwrap();

        if article_on_db.is_none() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let article = article_on_db.clone().unwrap();

        // checks user is allowed to perform the update
        let user_can_delete = verify_role_has_permission(
            &user_on_db
                .as_ref()
                .unwrap()
                .role()
                .unwrap()
                .clone()
                .to_owned(),
            RolePermissions::DeleteArticle,
        );

        if !user_can_delete {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let response = &self
            .article_comment_repository
            .delete_article_and_inactivate_comments(article)
            .await;

        if response.as_ref().is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Article Service, while deleting the article: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use tokio;
    use uuid::Uuid;

    use super::{DeleteArticleParams, DeleteArticleService};

    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_comment_repository::MockArticleCommentRepositoryTrait;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::article_repository::get_article_repository;

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();
        let (article_db, mocked_article_repo) = get_article_repository();
        let mut mocked_article_comment_repo = MockArticleCommentRepositoryTrait::new();

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string(),
            1,
            "Foo".into(),
        );

        article_db.lock().unwrap().push(article.clone());

        let db_clone = Arc::clone(&article_db);
        mocked_article_comment_repo
            .expect_delete_article_and_inactivate_comments()
            .returning(move |param_article| {
                let mut new_articles_db = vec![];

                for article in db_clone.lock().unwrap().iter() {
                    if article.id().ne(&param_article.id()) {
                        new_articles_db.push(article.clone());
                    }
                }

                *db_clone.lock().unwrap() = new_articles_db;
                Ok(())
            });

        mocked_user_repo.expect_find_by_id().returning(|id| {
            let fake_user = User::new_from_existing(
                id.clone().to_owned(),
                "Fake name".to_string(),
                "password".to_string(),
                TimeHelper::now(),
                None,
                Some(Role::Principal),
            );

            Ok(Some(fake_user))
        });

        let service = DeleteArticleService {
            user_repository: Box::new(mocked_user_repo),
            article_comment_repository: Box::new(mocked_article_comment_repo),
            article_repository: Box::new(mocked_article_repo),
        };

        let result = service
            .exec(DeleteArticleParams {
                user_id: article.author_id(),
                article_id: article.id(),
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(0, article_db.lock().unwrap().len());
    }
}
