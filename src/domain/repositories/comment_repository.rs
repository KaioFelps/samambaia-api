use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

use crate::domain::domain_entities::comment::Comment;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentRepositoryTrait {
    async fn create(&self, comment: Comment) -> Result<Comment, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>>;

    async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>>;

    async fn save(&self, comment: Comment) -> Result<Comment, Box<dyn Error>>;
}