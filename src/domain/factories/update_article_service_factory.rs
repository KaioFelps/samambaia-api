use crate::domain::services::update_article_service::UpdateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> UpdateArticleService<SeaArticleRepository> {
    let sea_service = SeaService::new().await;
    
    let article_repository = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let update_article_service = UpdateArticleService::new(
        article_repository,
    );

    update_article_service
}