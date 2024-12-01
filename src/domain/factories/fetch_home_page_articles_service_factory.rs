use crate::domain::services::fetch_home_page_articles_service::FetchHomePageArticlesService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> FetchHomePageArticlesService<SeaArticleRepository> {
    let article_repository = Box::new(SeaArticleRepository::new(db_conn).await);
    FetchHomePageArticlesService::new(article_repository)
}
