use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct HomeQueryDto {
    #[serde(rename = "fb_p")]
    pub free_badges_page: Option<u32>,
}
