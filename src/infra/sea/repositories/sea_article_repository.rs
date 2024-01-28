use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use std::error::Error;

use crate::domain::repositories::article_repositoy::ArticleRepositoryTrait;
use crate::domain::domain_entities::article::Article;
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaArticleRepository {
    sea_service: SeaService
}

impl SeaArticleRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaArticleRepository {
            sea_service: service.clone()
        }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let new_article = SeaArticleMapper::article_to_sea_active_model(article);

        let db = &self.sea_service.db;

        let created_article = new_article.insert(db).await.unwrap();
        let created_article = SeaArticleMapper::model_to_article(created_article);

        Ok(created_article)
    }
}

