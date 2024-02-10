use std::error::Error;

use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::{
    article_repository::ArticleRepositoryTrait,
    comment_repository::CommentRepositoryTrait,
    user_repository::UserRepositoryTrait
};
use crate::errors::{
    bad_request_error::BadRequestError,
    internal_error::InternalError,
    unauthorized_error::UnauthorizedError
};

pub struct CommentOnArticleParams {
    pub author_id: Uuid,
    pub article_id: Uuid,
    pub content: String
}

pub struct CommentOnArticleService<CR, AR, UR>
where CR: CommentRepositoryTrait, AR: ArticleRepositoryTrait, UR: UserRepositoryTrait
{
    comment_repository: Box<CR>,
    article_repository: Box<AR>,
    user_repository: Box<UR>
}

impl<
CR: CommentRepositoryTrait,
AR: ArticleRepositoryTrait,
UR: UserRepositoryTrait
>
CommentOnArticleService<CR, AR, UR> {
    pub fn new(
        comment_repository: Box<CR>,
        article_repository: Box<AR>,
        user_repository: Box<UR>
    ) -> Self {
        CommentOnArticleService {
            comment_repository,
            article_repository,
            user_repository
        }
    }

    pub async fn exec(&self, params: CommentOnArticleParams) -> Result<Comment, Box<dyn Error>> {
        let user_on_db = self.user_repository.find_by_id(&params.author_id).await;
        
        if user_on_db.is_err() {
            error!("\r\n===========\r\nError occurred on comment_on_article_service.rs, while fetching user from db:\r\n{:#?}\r\n===========\r\n", user_on_db.unwrap_err());
            return Err( Box::new( InternalError::new() ) );
        }
        if user_on_db.unwrap().is_none() { return Err( Box::new( UnauthorizedError::new() ) ); }
        
        let article_on_db = self.article_repository.find_by_id(params.article_id.clone()).await;

        if article_on_db.is_err() {
            error!("\r\n===========\r\nError occurred on comment_on_article_service.rs, while fetching article from db:\r\n{:#?}\r\n===========\r\n", article_on_db.unwrap_err());
            return Err( Box::new( InternalError::new() ) );
        }

        if article_on_db.unwrap().is_none() { return Err( Box::new( BadRequestError::new() ) ); }

        let comment = Comment::new(
            params.author_id,
            params.content,
        );

        let response = self.comment_repository.create(comment, params.article_id).await;

        if response.is_err() {
            error!("\r\n===========\r\nError occurred on comment_on_article_service.rs, while creating comment transaction:\r\n{:#?}\r\n===========\r\n", response.unwrap_err());
            return Err(Box::new(InternalError::new()));
        }

        Ok(response.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Mutex;
    use std::sync::Arc;
    
    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::slug::Slug;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;

    use chrono::Utc;

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct CommentArticle {
        pub id: Uuid,
        pub article_id: Uuid,
        pub comment_id: Uuid,
    }

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo = MockUserRepositoryTrait::new();
        let mut mocked_article_repo = MockArticleRepositoryTrait::new();
        let mut mocked_comment_repo = MockCommentRepositoryTrait::new();

        let user_id = Uuid::new_v4();
        let article_id = Uuid::new_v4();

        mocked_user_repo
        .expect_find_by_id()
        .returning(|id| {
            let user = User::new_from_existing(
                *id,
                "User".into(),
                "123456".into(),
                Utc::now().naive_utc(),
                None,
                Some(Role::User)
            );

            Ok(Some(user))
        });

        mocked_article_repo
        .expect_find_by_id()
        .returning(move |id| {
            let article = Article::new_from_existing(
                id.clone(),
                user_id.clone(),
                "cover_url".into(),
                "title".into(),
                "content".into(),
                false,
                Utc::now().naive_utc(),
                None,
                Slug::new(id, "title".into())
            );

            Ok(Some(article))
        });

        let comment_article_db: Arc<Mutex<Vec<CommentArticle>>> = Arc::new(Mutex::new(vec![]));
        let comment_db: Arc<Mutex<Vec<Comment>>> = Arc::new(Mutex::new(vec![]));

        let comment_article_db_move_clone = Arc::clone(&comment_article_db);
        let comment_db_move_clone = Arc::clone(&comment_db);

        mocked_comment_repo
        .expect_create()
        .returning(move |comment, article_id| {
            comment_article_db_move_clone.lock().unwrap().push(CommentArticle {
            id: Uuid::new_v4(),
            article_id,
            comment_id: comment.id()
            });

            comment_db_move_clone.lock().unwrap().push(comment.clone());

            Ok(comment)
        });

        let sut = CommentOnArticleService::new(
            Box::new(mocked_comment_repo),
            Box::new(mocked_article_repo),
            Box::new(mocked_user_repo)
        );

        let res = sut.exec(CommentOnArticleParams {
            article_id: article_id.clone(),
            author_id: user_id.clone(),
            content: "This article is awesome!".into()
        }).await;

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(
            (res.author_id() ,res.content()),
            (user_id, "This article is awesome!")
        );

        assert_eq!(1, comment_article_db.lock().unwrap().len());

        let relation_1 = comment_article_db.lock().unwrap()[0];

        assert_eq!(
            (relation_1.article_id, relation_1.comment_id),
            (article_id, res.id())
        );
    }
}