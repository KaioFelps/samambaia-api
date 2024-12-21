use crate::{
    domain::services::announcements::update_announcement_service::UpdateAnnouncementService,
    infra::sea::{
        repositories::{
            sea_announcement_repository::SeaAnnouncementRepository,
            sea_user_repository::SeaUserRepository,
        },
        sea_service::SeaService,
    },
};

pub async fn exec(
    db_conn: &SeaService,
) -> UpdateAnnouncementService<SeaUserRepository, SeaAnnouncementRepository> {
    let sea_user_repository = SeaUserRepository::new(db_conn).await;
    let sea_announcement_repository = SeaAnnouncementRepository::new(db_conn).await;

    UpdateAnnouncementService::new(sea_user_repository, sea_announcement_repository)
}
