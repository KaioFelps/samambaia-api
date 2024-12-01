use crate::domain::services::update_free_badge_service::UpdateFreeBadgeService;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> UpdateFreeBadgeService<SeaFreeBadgeRepository> {
    let free_badge_repository = SeaFreeBadgeRepository::new(db_conn).await;
    UpdateFreeBadgeService::new(free_badge_repository)
}
