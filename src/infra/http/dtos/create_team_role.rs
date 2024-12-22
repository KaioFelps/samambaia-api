use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTeamRoleDto {
    #[validate(length(
        min = 1,
        message = "Team role's title must be at least one string long."
    ))]
    pub title: String,
    #[validate(length(
        min = 1,
        message = "Team role's description must be at least one string long."
    ))]
    pub description: String,
}
