use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

use crate::domain::domain_entities::comment::Comment;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyCommentsResponse (
    pub Vec<Comment>, // data
    pub u64, // count
);

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentRepositoryTrait {
    async fn create(&self, comment: Comment, article_id: Uuid) -> Result<Comment, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>>;

    // async fn find_many_by_comment_id(&self, comment_id: Uuid) -> Result<FindManyCommentsResponse, Box<dyn Error>>;

    async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>>;
}