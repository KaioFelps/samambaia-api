use crate::domain::services::create_article_tag_service::CreateArticleTagService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<CreateArticleTagService<SeaArticleTagRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let sea_article_tag_repository: SeaArticleTagRepository = SeaArticleTagRepository::new(sea_service).await;

    let create_article_tag_service = CreateArticleTagService::new(
        sea_article_tag_repository,
    );

    Left(create_article_tag_service)
}