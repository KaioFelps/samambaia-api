use uuid::Uuid;

use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_comment_repository::ArticleCommentRepositoryTrait;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct DeleteArticleParams<'a> {
    pub user: &'a User,
    pub article_id: Uuid,
}
pub struct DeleteArticleService<AR: ArticleRepositoryTrait, ACR: ArticleCommentRepositoryTrait> {
    article_repository: AR,
    article_comment_repository: ACR,
}

impl<AR: ArticleRepositoryTrait, ACR: ArticleCommentRepositoryTrait> DeleteArticleService<AR, ACR> {
    pub fn new(article_repository: AR, article_comment_repository: ACR) -> Self {
        DeleteArticleService {
            article_repository,
            article_comment_repository,
        }
    }

    pub async fn exec(&self, params: DeleteArticleParams<'_>) -> Result<(), SamambaiaError> {
        let article = match self
            .article_repository
            .find_by_id(params.article_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Delete Article Service, while finding article by Id",
                    err,
                )
            })? {
            None => return Err(SamambaiaError::resource_not_found_err()),
            Some(article) => article,
        };

        // checks user is allowed to perform the update
        let user_can_delete = params.user.id().eq(&article.author_id())
            || verify_role_has_permission(
                &params.user.role().unwrap(),
                RolePermissions::DeleteArticle,
            );

        if !user_can_delete {
            return Err(SamambaiaError::unauthorized_err());
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
    use crate::domain::domain_entities::article_tag::DraftArticleTag;
    use crate::domain::domain_entities::comment::Comment;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::tests::relationship_managers::comment_article::CommentArticleRelationInMemoryManager;
    use crate::tests::repositories::article_comment_repository::get_article_comment_repository;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;

    #[tokio::test]
    async fn user_should_be_able_to_delete_own_article() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository);

        let relationship_manager = Arc::new(CommentArticleRelationInMemoryManager::new());
        let (article_db, _comment_db, article_comment_repository) = get_article_comment_repository(
            Some(article_repository.article_db.clone()),
            None,
            relationship_manager,
        );

        let user = User::new("Flori".into(), "".into(), Some(Role::Writer));
        let article = Article::new(
            user.id(),
            "Title".into(),
            "Content".into(),
            "cover".into(),
            "desc".into(),
            vec![],
            None,
        );

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(article.clone());

        let sut = DeleteArticleService::new(article_repository, article_comment_repository);
        let response = sut
            .exec(DeleteArticleParams {
                user: &user,
                article_id: article.id(),
            })
            .await;

        assert!(response.is_ok());
        assert_eq!(0, article_db.lock().unwrap().len());
    }

    #[tokio::test]
    async fn unauthorized_user_should_not_be_able_to_delete_others_articles() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let relationship_manager = Arc::new(CommentArticleRelationInMemoryManager::new());
        let (article_db, _comment_db, article_comment_repository) = get_article_comment_repository(
            Some(article_repository.article_db.clone()),
            None,
            relationship_manager,
        );

        let tag = article_tag_repository
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string(),
            "Bar baz!".into(),
            vec![tag],
            None,
        );

        let non_author_user = User::new("Flori".into(), "".into(), Some(Role::Admin));

        article_db.lock().unwrap().push(article.clone());

        let service = DeleteArticleService {
            article_comment_repository,
            article_repository,
        };

        let response = service
            .exec(DeleteArticleParams {
                user: &non_author_user,
                article_id: article.id(),
            })
            .await;

        assert!(response.is_err());
        assert_eq!(1, article_db.lock().unwrap().len());
    }

    #[tokio::test]
    async fn authorized_user_should_be_able_to_delete_anyones_article() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let relationship_manager = Arc::new(CommentArticleRelationInMemoryManager::new());
        let (article_db, _comment_db, article_comment_repository) = get_article_comment_repository(
            Some(article_repository.article_db.clone()),
            None,
            relationship_manager,
        );

        let tag = article_tag_repository
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string(),
            "Bar baz!".into(),
            vec![tag],
            None,
        );

        let principal = User::new("Flori".into(), "".into(), Some(Role::Principal));

        article_db.lock().unwrap().push(article.clone());

        let service = DeleteArticleService {
            article_comment_repository,
            article_repository,
        };

        let response = service
            .exec(DeleteArticleParams {
                user: &principal,
                article_id: article.id(),
            })
            .await;

        assert!(response.is_ok());
        assert_eq!(0, article_db.lock().unwrap().len());
    }

    #[tokio::test]
    async fn it_should_delete_comments_along_with_article() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let relationship_manager = Arc::new(CommentArticleRelationInMemoryManager::new());
        let (article_db, comment_db, article_comment_repository) = get_article_comment_repository(
            Some(article_repository.article_db.clone()),
            None,
            relationship_manager,
        );

        let tag = article_tag_repository
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string(),
            "Bar baz!".into(),
            vec![tag],
            None,
        );

        article_db.lock().unwrap().push(article.clone());

        let comment_db_clone = comment_db.clone();
        {
            let mut lock = comment_db_clone.lock().unwrap();
            lock.push(Comment::new(
                Uuid::new_v4(),
                Some(article.id()),
                "Foo".into(),
            ));
            lock.push(Comment::new(
                Uuid::new_v4(),
                Some(article.id()),
                "Bar".into(),
            ));
            lock.push(Comment::new(
                Uuid::new_v4(),
                Some(article.id()),
                "Baz".into(),
            ));
            lock.push(Comment::new(
                Uuid::new_v4(),
                Some(article.id()),
                "Foz".into(),
            ));

            assert_eq!(4, lock.len());
        }

        let principal = User::new("Flori".into(), "".into(), Some(Role::Principal));
        let sut = DeleteArticleService::new(article_repository, article_comment_repository);

        let response = sut
            .exec(DeleteArticleParams {
                user: &principal,
                article_id: article.id(),
            })
            .await;

        assert!(response.is_ok());
        assert_eq!(0, article_db.lock().unwrap().len());
        assert_eq!(
            0,
            comment_db
                .lock()
                .unwrap()
                .iter()
                .filter(|comment| comment.is_active())
                .collect::<Vec<_>>()
                .len()
        );
    }
}
