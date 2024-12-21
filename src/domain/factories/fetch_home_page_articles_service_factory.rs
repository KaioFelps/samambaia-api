use crate::domain::services::journalism::articles::fetch_home_page_articles_service::FetchHomePageArticlesService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FetchHomePageArticlesService<SeaArticleRepository> {
    let article_repository = SeaArticleRepository::new(db_conn);
    FetchHomePageArticlesService::new(article_repository)
}
