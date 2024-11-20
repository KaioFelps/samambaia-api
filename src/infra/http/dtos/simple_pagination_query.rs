use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct SimplePaginationQueryDto {
    pub page: Option<u32>,

    #[serde(rename = "perPage")]
    pub per_page: Option<u8>,
}
