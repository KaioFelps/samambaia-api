use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ListArticlesDto {
    pub page: Option<u32>,

    #[serde(rename = "perPage")]
    pub per_page: Option<u8>,

    pub title: Option<String>,

    pub author: Option<String>,
}
