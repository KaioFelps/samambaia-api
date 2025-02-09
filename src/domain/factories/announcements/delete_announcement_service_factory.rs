use crate::domain::services::announcements::delete_announcement_service::DeleteAnnouncementService;
use crate::infra::sea::repositories::sea_announcement_repository::SeaAnnouncementRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> DeleteAnnouncementService<SeaUserRepository, SeaAnnouncementRepository> {
    let sea_user_repository = SeaUserRepository::new(db_conn);
    let sea_announcement_repository = SeaAnnouncementRepository::new(db_conn);

    DeleteAnnouncementService::new(sea_user_repository, sea_announcement_repository)
}
