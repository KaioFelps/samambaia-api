use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTeamUserDto {
    pub nickname: String,

    #[serde(rename = "function")]
    pub user_function: String,

    #[validate(url(message = "Twitter/X field must be a valid url."))]
    pub twitter: Option<String>,

    #[validate(url(message = "Discord field must be a valid url."))]
    pub discord: Option<String>,

    pub team_role_id: Uuid,
}
