use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

use crate::domain::domain_entities::{article::Article, comment::Comment};
 
#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyCommentsResponse (
    pub Vec<Comment>, // data
    pub u64, // count
);

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleCommentRepositoryTrait {
    async fn find_many_comments_by_article_id(& self, article_id: Uuid) -> Result<FindManyCommentsResponse, Box<dyn Error>>;

    async fn delete_many_comments_by_article_id(& self, article_id: Uuid) -> Result<(), Box<dyn Error>>;

    async fn delete_article_with_comments(& self, article: Article) -> Result<(), Box<dyn Error>>;
}
