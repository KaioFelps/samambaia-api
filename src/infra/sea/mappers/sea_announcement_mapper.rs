use entities::announcement::{ActiveModel as AnnouncementActiveModel, Model as AnnouncementModel};
use sea_orm::IntoActiveValue;

use super::SeaMapper;
use crate::domain::domain_entities::announcement::Announcement;

pub struct SeaAnnouncementMapper;

impl SeaMapper<Announcement, AnnouncementModel, AnnouncementActiveModel> for SeaAnnouncementMapper {
    fn entity_into_model(entity: Announcement) -> AnnouncementModel {
        AnnouncementModel {
            author_id: *entity.author_id(),
            created_at: *entity.created_at(),
            description: entity.description().to_owned(),
            external: *entity.external(),
            id: *entity.id(),
            image: entity.image().to_owned(),
            updated_at: *entity.updated_at(),
            url: entity.url().to_owned(),
        }
    }

    fn entity_into_active_model(entity: Announcement) -> AnnouncementActiveModel {
        AnnouncementActiveModel {
            id: entity.id().into_active_value(),
            author_id: entity.author_id().into_active_value(),
            description: entity.description().to_owned().into_active_value(),
            url: entity.url().to_owned().into_active_value(),
            image: entity.image().to_owned().into_active_value(),
            external: entity.external().into_active_value(),
            created_at: entity.created_at().into_active_value(),
            updated_at: entity.updated_at().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: AnnouncementActiveModel) -> Announcement {
        Announcement::new_from_existing(
            active_model.id.unwrap(),
            active_model.url.unwrap(),
            active_model.image.unwrap(),
            active_model.external.unwrap(),
            active_model.description.unwrap(),
            active_model.created_at.unwrap(),
            active_model.updated_at.unwrap(),
            active_model.author_id.unwrap(),
        )
    }

    fn model_into_entity(model: AnnouncementModel) -> Announcement {
        Announcement::new_from_existing(
            model.id,
            model.url,
            model.image,
            model.external,
            model.description,
            model.created_at,
            model.updated_at,
            model.author_id,
        )
    }
}
