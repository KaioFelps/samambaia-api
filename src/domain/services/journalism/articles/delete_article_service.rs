use uuid::Uuid;

use crate::domain::repositories::article_comment_repository::ArticleCommentRepositoryTrait;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::DomainError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct DeleteArticleParams {
    pub user_id: Uuid,
    pub article_id: Uuid,
}
pub struct DeleteArticleService<
    AR: ArticleRepositoryTrait,
    ACR: ArticleCommentRepositoryTrait,
    UR: UserRepositoryTrait,
> {
    article_repository: AR,
    article_comment_repository: ACR,
    user_repository: UR,
}

impl<AR: ArticleRepositoryTrait, ACR: ArticleCommentRepositoryTrait, UR: UserRepositoryTrait>
    DeleteArticleService<AR, ACR, UR>
{
    pub fn new(
        article_repository: AR,
        article_comment_repository: ACR,
        user_repository: UR,
    ) -> Self {
        DeleteArticleService {
            article_repository,
            article_comment_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: DeleteArticleParams) -> Result<(), DomainError> {
        let user_on_db = self
            .user_repository
            .find_by_id(&params.user_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Delete Article Service, while finding user by Id",
                    err,
                )
            })?;

        if user_on_db.is_none() {
            return Err(DomainError::unauthorized_err());
        }

        // article verifications

        let article_on_db = self
            .article_repository
            .find_by_id(params.article_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Delete Article Service, while finding article by Id",
                    err,
                )
            })?;

        if article_on_db.is_none() {
            return Err(DomainError::resource_not_found_err());
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
            return Err(DomainError::unauthorized_err());
        }

        self.article_comment_repository
            .delete_article_and_inactivate_comments(article)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Delete Article Service, while deleting the article",
                    err,
                )
            })
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
        let (article_db, _, mocked_article_repo) = get_article_repository();
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
            user_repository: mocked_user_repo,
            article_comment_repository: mocked_article_comment_repo,
            article_repository: mocked_article_repo,
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
