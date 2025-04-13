use std::sync::{Arc, Mutex};

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;
use crate::tests::relationship_managers::comment_article::{
    CommentArticle,
    CommentArticleRelationInMemoryManager,
};

pub fn get_comment_repository(
    db: Option<Arc<Mutex<Vec<Comment>>>>,
    relationship_manager: Arc<CommentArticleRelationInMemoryManager>,
) -> (Arc<Mutex<Vec<Comment>>>, MockCommentRepositoryTrait) {
    let db = db.unwrap_or(Arc::new(Mutex::new(vec![])));
    let mut repository = MockCommentRepositoryTrait::new();

    let db_c = db.clone();
    repository.expect_create().returning(move |comment| {
        relationship_manager
            .db
            .lock()
            .unwrap()
            .push(CommentArticle {
                article_id: comment.article_id().unwrap(),
                comment_id: comment.id(),
            });

        db_c.lock().unwrap().push(comment.clone());

        Ok(comment)
    });

    (db, repository)
}
