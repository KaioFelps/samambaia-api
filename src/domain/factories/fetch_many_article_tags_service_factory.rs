use crate::domain::services::fetch_many_article_tags_service::FetchManyArticleTagsService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchManyArticleTagsService<SeaArticleTagRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    let article_tag_repository = SeaArticleTagRepository::new(sea_service).await;

    let fetch_many_article_tags_service = FetchManyArticleTagsService::new(article_tag_repository);

    Left(fetch_many_article_tags_service)
}
