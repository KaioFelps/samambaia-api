use crate::domain::services::announcements::fetch_many_announcements_service::FetchManyAnnouncementsService;
use crate::infra::sea::repositories::sea_announcement_repository::SeaAnnouncementRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FetchManyAnnouncementsService<SeaAnnouncementRepository> {
    let sea_announcement_repository = SeaAnnouncementRepository::new(db_conn);
    FetchManyAnnouncementsService::new(sea_announcement_repository)
}
