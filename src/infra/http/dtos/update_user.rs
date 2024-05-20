use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::domain_entities::role::Role;
use crate::core::NICKNAME_REGX;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(
        regex(path= "NICKNAME_REGX", message = "Your nickname might contain only letters, numbers, and the symbols: .,_-=?!@:;."),
        length(min = 3, max = 22, message = "Your nickname must be between 3 and 22 chars.")
    )]
    pub nickname: Option<String>,

    #[validate(length(min = 1))]
    pub password: Option<String>,

    pub role: Option<Role>
}
