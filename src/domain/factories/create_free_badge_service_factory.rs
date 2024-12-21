use crate::domain::services::create_free_badge_service::CreateFreeBadgeService;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> CreateFreeBadgeService<SeaFreeBadgeRepository> {
    let free_badge_repository = SeaFreeBadgeRepository::new(db_conn);
    CreateFreeBadgeService::new(free_badge_repository)
}
