use entities::free_badge::Model as FreeBadgeModel;
use entities::free_badge::ActiveModel as FreeBadgeActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::free_badge::FreeBadge;

pub struct SeaFreeBadgeMapper;

impl SeaFreeBadgeMapper {
    pub fn free_badge_to_sea_model(free_badge: FreeBadge) -> FreeBadgeModel {
        FreeBadgeModel {
            id: free_badge.id(),
            image: free_badge.image().into(),
            code: free_badge.code().into(),
            link: free_badge.link().into(),
            link_is_external: free_badge.link_is_external(),
            created_at: free_badge.created_at(),
            available_until: free_badge.available_until(),
        }
    }

    pub fn free_badge_to_sea_active_model(free_badge: FreeBadge) -> FreeBadgeActiveModel {
        FreeBadgeActiveModel {
            id: free_badge.id().into_active_value(),
            image: free_badge.image().to_owned().into_active_value(),
            code: free_badge.code().to_owned().into_active_value(),
            link: free_badge.link().to_owned().into_active_value(),
            link_is_external: free_badge.link_is_external().into_active_value(),
            created_at: free_badge.created_at().into_active_value(),
            available_until: free_badge.available_until().into_active_value(),
        }
    }

    pub fn active_model_to_free_badge(active_model_free_badge: FreeBadgeActiveModel) -> FreeBadge {
        FreeBadge::new_from_existing(
            active_model_free_badge.id.unwrap(),
            active_model_free_badge.code.unwrap(),
            active_model_free_badge.image.unwrap(),
            active_model_free_badge.link.unwrap(),
            active_model_free_badge.link_is_external.unwrap(),
            active_model_free_badge.created_at.unwrap(),
            active_model_free_badge.available_until.unwrap(),
        )
    }

    pub fn model_to_free_badge(model_free_badge: FreeBadgeModel) -> FreeBadge {
        FreeBadge::new_from_existing(
            model_free_badge.id.into(),
            model_free_badge.code.into(),
            model_free_badge.image.into(),
            model_free_badge.link.into(),
            model_free_badge.link_is_external.into(),
            model_free_badge.created_at.into(),
            model_free_badge.available_until.into()
        )
    }
}