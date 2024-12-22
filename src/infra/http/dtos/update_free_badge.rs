use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateFreeBadgeDto {
    pub code: Option<String>,
    pub link: Option<String>,
    #[serde(rename = "linkIsExternal")]
    pub link_is_external: Option<bool>,
    #[serde(rename = "availableUntil")]
    pub available_until: Option<Option<NaiveDateTime>>,
    #[validate(url(message = "Free badge image field's value must be a valid URL."))]
    pub image: Option<String>,
}
