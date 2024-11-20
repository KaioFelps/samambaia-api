use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ListTeamUsersDto {
    pub page: Option<u32>,

    pub per_page: Option<u8>,

    pub team_role_id: Option<Uuid>,

    #[validate(length(min = 1))]
    pub nickname: Option<String>,
}
