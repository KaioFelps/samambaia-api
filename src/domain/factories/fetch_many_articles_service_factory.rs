use crate::domain::services::fetch_many_articles_service::FetchManyArticlesService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> FetchManyArticlesService<SeaArticleRepository, SeaUserRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let article_repository: Box<SeaArticleRepository> = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let fetch_many_articles_service = FetchManyArticlesService::new(article_repository, user_repository);

    fetch_many_articles_service
}