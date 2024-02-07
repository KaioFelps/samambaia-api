use crate::domain::services::fetch_home_page_articles_service::FetchHomePageArticlesService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> FetchHomePageArticlesService<SeaArticleRepository, > {   
    let sea_service = SeaService::new().await;
    let article_repository = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let fetch_home_page_articles_service = FetchHomePageArticlesService::new(article_repository);

    fetch_home_page_articles_service
}