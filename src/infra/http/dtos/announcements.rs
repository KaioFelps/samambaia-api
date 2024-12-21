use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateAnnouncementDto {
    #[validate(url(message = "url field must contain a valid url."))]
    pub url: String,

    #[validate(url(message = "image field must contain a valid image url."))]
    pub image: String,

    pub external: bool,

    pub description: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateAnnouncementDto {
    #[validate(url(message = "url field must contain a valid url."))]
    pub url: Option<String>,

    #[validate(url(message = "image field must contain a valid image url."))]
    pub image: Option<String>,

    pub external: Option<bool>,

    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct ListAnnouncementsDto {
    pub page: Option<u32>,

    #[serde(rename = "perPage")]
    pub per_page: Option<u8>,

    pub description: Option<String>,
}
