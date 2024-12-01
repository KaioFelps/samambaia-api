use crate::domain::services::fetch_many_free_badges_service::FetchManyFreeBadgesService;
use crate::infra::sea::repositories::sea_free_badge_repository::SeaFreeBadgeRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> FetchManyFreeBadgesService<SeaFreeBadgeRepository> {
    let free_badge_repository = SeaFreeBadgeRepository::new(db_conn).await;
    FetchManyFreeBadgesService::new(free_badge_repository)
}
