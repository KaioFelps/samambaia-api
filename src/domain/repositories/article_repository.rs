use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

use crate::domain::domain_entities::article::Article;
use crate::core::pagination::PaginationParameters;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyResponse (
    pub Vec<Article>,
    pub u64,
);

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleRepositoryTrait {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>>;

    async fn find_many(&self, params: PaginationParameters) -> Result<FindManyResponse, Box<dyn Error>>;

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>>;

    async fn delete(&self, article: Article) -> Result<(), Box<dyn Error>>;
}