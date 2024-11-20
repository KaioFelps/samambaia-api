use crate::core::NICKNAME_REGX;
use crate::libs::custom_validators::validate_user_role;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(
        regex(path= *NICKNAME_REGX, message = "Your nickname might contain only letters, numbers, and the symbols: .,_-=?!@:;."),
        length(min = 3, max = 22, message = "Your nickname must be between 3 and 22 chars.")
    )]
    pub nickname: Option<String>,

    #[validate(length(min = 1))]
    pub password: Option<String>,

    #[validate(custom(function = "validate_user_role"))]
    pub role: Option<String>,
}
