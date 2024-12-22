use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ListCommentReportsDto {
    pub page: Option<u32>,

    #[serde(rename = "perPage")]
    pub per_page: Option<u8>,

    #[serde(rename = "solvedBy")]
    pub solved_by: Option<String>,

    pub solved: Option<bool>,

    pub content: Option<String>,
}
