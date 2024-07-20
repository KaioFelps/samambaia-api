use std::error::Error;

use async_trait::async_trait;
use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
use crate::core::pagination::PaginationParameters;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyArticleTagsResponse (
    pub Vec<ArticleTag>,
    pub u64,
);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArticleTagQueryType {
    Title(String),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleTagRepositoryTrait {
    async fn create(&self, article_tag: DraftArticleTag) -> Result<ArticleTag, Box<dyn Error>>;

    async fn find_by_id(&self, tag_id: i32) -> Result<Option<ArticleTag>, Box<dyn Error>>;

    async fn find_by_value(&self, tag_value: String) -> Result<Option<ArticleTag>, Box<dyn Error>>;

    async fn find_many(&self, params: PaginationParameters<ArticleTagQueryType>) -> Result<FindManyArticleTagsResponse, Box<dyn Error>>;

    async fn save(&self, article_tag: ArticleTag) -> Result<ArticleTag, Box<dyn Error>>;

    async fn delete(&self, article_tag: ArticleTag) -> Result<(), Box<dyn Error>>;
}
