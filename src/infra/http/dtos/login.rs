use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginDto {
    pub nickname: String,

    #[validate(length(min = 1))]
    pub password: String,
}
