use crate::core::pagination::PaginationParameters;
use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyCommentsWithAuthorResponse(
    pub Vec<CommentWithAuthor>, // data
    pub u64,                    // count
);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CommentWithAuthorQueryType {
    Author(Uuid),
    Content(String),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentUserArticleRepositoryTrait {
    async fn find_many_comments(
        &self,
        article_id: Uuid,
        include_inactive: bool,
        params: PaginationParameters<CommentWithAuthorQueryType>,
    ) -> Result<FindManyCommentsWithAuthorResponse, Box<dyn Error>>;
}
