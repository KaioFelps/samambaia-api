use async_trait::async_trait;
use crate::{domain::repositories::article_repositoy::ArticleRepositoryTrait, infra::sea::sea_service::SeaService};

pub struct SeaArticleRepository {
    sea_service: SeaService
}

impl SeaArticleRepository {
    // constructor
    pub async fn new(service: &SeaService) -> Self {
        SeaArticleRepository {
            sea_service: service.clone()
        }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository {
}

