use async_trait::async_trait;
use std::error::Error;

#[cfg(test)]
use mockall::automock;

use crate::domain::domain_entities::team_role::TeamRole;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TeamRoleRepositoryTrait {
    async fn create(&self, team_role: TeamRole) -> Result<TeamRole, Box<dyn Error>>;
}
