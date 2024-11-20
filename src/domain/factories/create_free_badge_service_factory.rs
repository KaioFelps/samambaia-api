use crate::domain::services::create_free_badge_service::CreateFreeBadgeService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<CreateFreeBadgeService<SeaFreeBadgeRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;

    let create_free_badge_service = CreateFreeBadgeService::new(free_badge_repository);

    Left(create_free_badge_service)
}
