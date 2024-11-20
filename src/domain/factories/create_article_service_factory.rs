use crate::domain::services::create_article_service::CreateArticleService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<
    CreateArticleService<SeaArticleRepository, SeaArticleTagRepository, SeaUserRepository>,
    HttpResponse,
> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let sea_article_repository: Box<SeaArticleRepository> =
        Box::new(SeaArticleRepository::new(sea_service.clone()).await);
    let sea_article_tag_repository: Box<SeaArticleTagRepository> =
        Box::new(SeaArticleTagRepository::new(sea_service.clone()).await);
    let sea_user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let create_article_service = CreateArticleService::new(
        sea_article_repository,
        sea_article_tag_repository,
        sea_user_repository,
    );

    Left(create_article_service)
}
