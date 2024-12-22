use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTeamRoleDto {
    #[validate(length(
        min = 1,
        message = "Team role's title must be at least one string long."
    ))]
    pub title: Option<String>,
    #[validate(length(
        min = 1,
        message = "Team role's description must be at least one string long."
    ))]
    pub description: Option<String>,
}
