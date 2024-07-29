use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::domain::services::update_free_badge_service::UpdateFreeBadgeService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<UpdateFreeBadgeService<SeaFreeBadgeRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;

    let update_free_badge_service = UpdateFreeBadgeService::new(free_badge_repository);

    Left(update_free_badge_service)
}
