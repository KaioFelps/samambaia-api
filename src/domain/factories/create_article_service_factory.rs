use crate::domain::services::create_article_service::CreateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> CreateArticleService<SeaArticleRepository> {
    let sea_service = SeaService::new().await;

    let sea_article_repository: Box<SeaArticleRepository> = Box::new(SeaArticleRepository::new(sea_service).await);

    let create_article_service = CreateArticleService::new(
        sea_article_repository,
    );

    create_article_service
}