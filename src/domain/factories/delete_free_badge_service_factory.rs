use crate::domain::services::delete_free_badge_service::DeleteFreeBadgeService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<DeleteFreeBadgeService<SeaFreeBadgeRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;

    let delete_free_badge_service = DeleteFreeBadgeService::new(free_badge_repository);

    Ok(delete_free_badge_service)
}
