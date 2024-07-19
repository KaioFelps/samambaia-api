use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTeamUserDto {
    pub nickname: Option<String>,

    #[serde(rename="function")]
    pub user_function: Option<String>,

    #[validate(url(message = "Twitter/X field must be a valid url."))]
    pub twitter: Option<Option<String>>,

    #[validate(url(message = "Discord field must be a valid url."))]
    pub discord: Option<Option<String>>,

    pub team_role_id: Option<Uuid>,
}
