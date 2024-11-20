use crate::domain::services::create_free_badge_service::CreateFreeBadgeService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<CreateFreeBadgeService<SeaFreeBadgeRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let free_badge_repository = SeaFreeBadgeRepository::new(sea_service).await;

    let create_free_badge_service = CreateFreeBadgeService::new(free_badge_repository);

    Ok(create_free_badge_service)
}
