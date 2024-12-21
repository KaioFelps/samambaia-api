use crate::{
    domain::services::announcements::create_announcement_service::CreateAnnouncementService,
    infra::sea::{
        repositories::{
            sea_announcement_repository::SeaAnnouncementRepository,
            sea_user_repository::SeaUserRepository,
        },
        sea_service::SeaService,
    },
};

pub fn exec(
    db_conn: &SeaService,
) -> CreateAnnouncementService<SeaUserRepository, SeaAnnouncementRepository> {
    let sea_user_repository = SeaUserRepository::new(db_conn);
    let sea_announcement_repository = SeaAnnouncementRepository::new(db_conn);

    CreateAnnouncementService::new(sea_user_repository, sea_announcement_repository)
}
