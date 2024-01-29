use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

use crate::domain::domain_entities::article::Article;

#[cfg(test)]
use mockall::automock;


#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleRepositoryTrait {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>>;

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>>;

    async fn delete(&self, article: Article) -> Result<(), Box<dyn Error>>;
}