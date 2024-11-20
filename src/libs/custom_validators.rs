use std::str::FromStr;
use validator::ValidationError;

use crate::domain::domain_entities::role::Role;

pub fn validate_user_role(role: &str) -> Result<(), ValidationError> {
    let role_is_valid = Role::from_str(role).is_ok();

    match role_is_valid {
        true => Ok(()),
        false => Err(ValidationError::new("Invalid user role.")),
    }
}
