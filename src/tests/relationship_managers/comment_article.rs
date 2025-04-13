use std::sync::Mutex;

use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct CommentArticle {
    pub article_id: Uuid,
    pub comment_id: Uuid,
}

pub struct CommentArticleRelationInMemoryManager {
    pub db: Mutex<Vec<CommentArticle>>,
}

impl CommentArticleRelationInMemoryManager {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(Vec::new()),
        }
    }
}
