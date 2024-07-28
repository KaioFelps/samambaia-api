use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateFreeBadgeDto {
    pub code: String,
    pub link: String,
    #[serde(rename="linkIsExternal")]
    pub link_is_external: bool,
    #[serde(rename="availableUntil")]
    pub available_until: Option<NaiveDateTime>,
    #[validate(url(message = "Twitter/X field must be a valid url."))]
    pub image: String
}
