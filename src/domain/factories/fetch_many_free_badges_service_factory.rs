use crate::domain::services::fetch_many_free_badges_service::FetchManyFreeBadgesService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<FetchManyFreeBadgesService<SeaFreeBadgeRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;
    let fetch_many_free_badges_service = FetchManyFreeBadgesService::new(free_badge_repository);

    Ok(fetch_many_free_badges_service)
}
