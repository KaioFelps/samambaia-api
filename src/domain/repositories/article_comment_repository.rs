use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment::Comment;

#[derive(Debug)]
pub struct FindManyCommentsResponse(
    pub Vec<Comment>, // data
    pub u64,          // count
);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CommentQueryType {
    Author(Uuid),
    Content(String),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleCommentRepositoryTrait {
    async fn find_many_comments(
        &self,
        article_id: Option<Uuid>,
        include_inactive: bool,
        params: PaginationParameters<CommentQueryType>,
    ) -> Result<FindManyCommentsResponse, Box<dyn Error>>;

    async fn delete_many_comments_by_article_id(
        &self,
        article_id: Uuid,
    ) -> Result<(), Box<dyn Error>>;

    async fn inactivate_many_comments_by_article_id(
        &self,
        article_id: Uuid,
    ) -> Result<(), Box<dyn Error>>;

    async fn delete_article_with_comments(&self, article: Article) -> Result<(), Box<dyn Error>>;

    async fn delete_article_and_inactivate_comments(
        &self,
        article: Article,
    ) -> Result<(), Box<dyn Error>>;
}
