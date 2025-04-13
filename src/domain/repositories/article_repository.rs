use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::aggregations::article_preview::ArticlePreview;
use crate::domain::domain_entities::article::Article;
use crate::domain::value_objects::slug::Slug;

#[derive(Debug)]
pub struct FindManyArticlesResponse(pub Vec<Article>, pub u64);
pub struct FindManyArticlesPreviewsResponse(pub Vec<ArticlePreview>, pub u64);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArticleQueryType {
    Title(String),
    Tag(i32),
    Author(Uuid),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleRepositoryTrait {
    async fn create(&self, mut article: Article) -> Result<Article, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>>;

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, Box<dyn Error>>;

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesResponse, Box<dyn Error>>;

    async fn find_many_previews(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesPreviewsResponse, Box<dyn Error>>;

    async fn save(&self, mut article: Article) -> Result<Article, Box<dyn Error>>;
}
