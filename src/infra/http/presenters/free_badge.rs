use crate::domain::domain_entities::free_badge::FreeBadge;
use crate::infra::http::presenters::presenter::PresenterTrait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MappedFreeBadge {
    pub id: Uuid,
    pub code: String,
    pub link: String,
    #[serde(rename = "linkIsExternal")]
    pub link_is_external: bool,
    #[serde(rename = "availableUntil")]
    pub available_until: Option<NaiveDateTime>,
    pub image: String,
}

pub struct FreeBadgePresenter;

impl PresenterTrait<FreeBadge, MappedFreeBadge> for FreeBadgePresenter {
    fn to_http(free_badge: FreeBadge) -> MappedFreeBadge {
        MappedFreeBadge {
            id: free_badge.id(),
            code: free_badge.code().into(),
            link: free_badge.link().into(),
            image: free_badge.image().into(),
            available_until: free_badge.available_until(),
            link_is_external: free_badge.link_is_external(),
        }
    }
}
