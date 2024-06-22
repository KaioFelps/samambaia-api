use crate::domain::services::fetch_home_page_articles_service::FetchHomePageArticlesService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchHomePageArticlesService<SeaArticleRepository>, HttpResponse> {   
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    let article_repository = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let fetch_home_page_articles_service = FetchHomePageArticlesService::new(article_repository);

    Left(fetch_home_page_articles_service)
}