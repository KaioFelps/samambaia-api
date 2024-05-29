use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ChangePasswordDto {
    #[validate(length(min = 1))]
    pub current_password: String,

    #[validate(length(min = 1))]
    pub new_password: String,
}
