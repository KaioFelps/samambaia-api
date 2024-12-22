use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateFreeBadgeDto {
    pub code: String,
    pub link: String,
    #[serde(rename = "linkIsExternal")]
    pub link_is_external: bool,
    #[serde(rename = "availableUntil")]
    pub available_until: Option<NaiveDateTime>,
    #[validate(url(message = "Free badge image field's value must be a valid URL."))]
    pub image: String,
}
