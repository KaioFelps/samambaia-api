use crate::{
    domain::services::announcements::fetch_many_announcements_service::FetchManyAnnouncementsService,
    infra::sea::{
        repositories::sea_announcement_repository::SeaAnnouncementRepository,
        sea_service::SeaService,
    },
};

pub async fn exec(
    db_conn: &SeaService,
) -> FetchManyAnnouncementsService<SeaAnnouncementRepository> {
    let sea_announcement_repository = SeaAnnouncementRepository::new(db_conn).await;
    FetchManyAnnouncementsService::new(sea_announcement_repository)
}
