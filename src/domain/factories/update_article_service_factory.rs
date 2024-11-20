use crate::domain::services::update_article_service::UpdateArticleService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec(
) -> Either<UpdateArticleService<SeaArticleRepository, SeaArticleTagRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let article_repository = Box::new(SeaArticleRepository::new(sea_service.clone()).await);
    let article_tag_repository = Box::new(SeaArticleTagRepository::new(sea_service).await);

    let update_article_service =
        UpdateArticleService::new(article_repository, article_tag_repository);

    Left(update_article_service)
}
