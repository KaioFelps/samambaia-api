use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::slug::Slug;
use crate::infra::http::presenters::home_article::MappedHomeArticle;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyArticlesResponse(pub Vec<Article>, pub u64);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArticleQueryType {
    Title(String),
    Tag(i32),
    Author(Uuid),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleRepositoryTrait {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>>;

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, Box<dyn Error>>;

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesResponse, Box<dyn Error>>;

    async fn get_home_articles(&self) -> Result<Vec<MappedHomeArticle>, Box<dyn Error>>;

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>>;
}
