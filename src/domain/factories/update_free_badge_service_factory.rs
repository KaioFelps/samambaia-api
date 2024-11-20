use crate::domain::services::update_free_badge_service::UpdateFreeBadgeService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<UpdateFreeBadgeService<SeaFreeBadgeRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;

    let update_free_badge_service = UpdateFreeBadgeService::new(free_badge_repository);

    Ok(update_free_badge_service)
}
