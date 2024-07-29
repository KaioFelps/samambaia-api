use crate::domain::services::fetch_many_free_badges_service::FetchManyFreeBadgesService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchManyFreeBadgesService<SeaFreeBadgeRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service.unwrap()).await;
    let fetch_many_free_badges_service = FetchManyFreeBadgesService::new(free_badge_repository);

    Left(fetch_many_free_badges_service)
}