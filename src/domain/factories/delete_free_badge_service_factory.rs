
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::domain::services::delete_free_badge_service::DeleteFreeBadgeService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<DeleteFreeBadgeService<SeaFreeBadgeRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;

    let delete_free_badge_service = DeleteFreeBadgeService::new(free_badge_repository);

    Left(delete_free_badge_service)
}