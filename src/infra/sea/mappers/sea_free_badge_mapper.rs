use entities::free_badge::{ActiveModel as FreeBadgeActiveModel, Model as FreeBadgeModel};
use sea_orm::IntoActiveValue;

use super::SeaMapper;
use crate::domain::domain_entities::free_badge::FreeBadge;

pub struct SeaFreeBadgeMapper;

impl SeaMapper<FreeBadge, FreeBadgeModel, FreeBadgeActiveModel> for SeaFreeBadgeMapper {
    fn entity_into_model(entity: FreeBadge) -> FreeBadgeModel {
        FreeBadgeModel {
            id: entity.id(),
            image: entity.image().into(),
            code: entity.code().into(),
            link: entity.link().into(),
            link_is_external: entity.link_is_external(),
            created_at: entity.created_at(),
            available_until: entity.available_until(),
        }
    }

    fn entity_into_active_model(entity: FreeBadge) -> FreeBadgeActiveModel {
        FreeBadgeActiveModel {
            id: entity.id().into_active_value(),
            image: entity.image().to_owned().into_active_value(),
            code: entity.code().to_owned().into_active_value(),
            link: entity.link().to_owned().into_active_value(),
            link_is_external: entity.link_is_external().into_active_value(),
            created_at: entity.created_at().into_active_value(),
            available_until: entity.available_until().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: FreeBadgeActiveModel) -> FreeBadge {
        FreeBadge::new_from_existing(
            active_model.id.unwrap(),
            active_model.code.unwrap(),
            active_model.image.unwrap(),
            active_model.link.unwrap(),
            active_model.link_is_external.unwrap(),
            active_model.created_at.unwrap(),
            active_model.available_until.unwrap(),
        )
    }

    fn model_into_entity(model: FreeBadgeModel) -> FreeBadge {
        FreeBadge::new_from_existing(
            model.id,
            model.code,
            model.image,
            model.link,
            model.link_is_external,
            model.created_at,
            model.available_until,
        )
    }
}
