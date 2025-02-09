use serde::Serialize;
use uuid::Uuid;

use crate::domain::domain_entities::announcement::Announcement;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize)]
pub struct MappedAnnouncement {
    id: Uuid,
    url: String,
    image: String,
    external: bool,
    description: String,
}

pub struct AnnouncementPresenter;

impl PresenterTrait<Announcement, MappedAnnouncement> for AnnouncementPresenter {
    fn to_http(entity: Announcement) -> MappedAnnouncement {
        MappedAnnouncement {
            description: entity.description().to_owned(),
            external: *entity.external(),
            id: *entity.id(),
            image: entity.image().to_owned(),
            url: entity.url().to_owned(),
        }
    }
}
