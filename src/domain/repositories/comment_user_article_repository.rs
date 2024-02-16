use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;
use crate::core::pagination::PaginationParameters;

use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
 
#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyCommentsWithAuthorResponse (
    pub Vec<CommentWithAuthor>, // data
    pub u64, // count
);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CommentWithAuthorQueryType {
    AUTHOR,
    CONTENT,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentUserArticleRepositoryTrait {
    async fn find_many_comments(
        &self,
        article_id: Option<Uuid>,
        params: PaginationParameters<CommentWithAuthorQueryType>
    ) -> Result<FindManyCommentsWithAuthorResponse, Box<dyn Error>>;
}
