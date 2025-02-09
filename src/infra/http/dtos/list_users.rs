use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::custom_validators::validate_user_role;

#[derive(Serialize, Deserialize, Validate)]
pub struct ListUsersDto {
    pub page: Option<u32>,

    pub per_page: Option<u8>,

    #[validate(custom(function = "validate_user_role"))]
    pub role: Option<String>,

    #[validate(length(min = 1))]
    pub nickname: Option<String>,
}
