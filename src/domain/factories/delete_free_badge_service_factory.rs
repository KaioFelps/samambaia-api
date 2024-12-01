use crate::domain::services::delete_free_badge_service::DeleteFreeBadgeService;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> DeleteFreeBadgeService<SeaFreeBadgeRepository> {
    let free_badge_repository = SeaFreeBadgeRepository::new(db_conn).await;
    DeleteFreeBadgeService::new(free_badge_repository)
}
