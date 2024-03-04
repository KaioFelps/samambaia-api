use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

#[cfg(test)]
use mockall::automock;

use crate::domain::domain_entities::team_role::TeamRole;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TeamRoleRepositoryTrait {
    async fn create(&self, team_role: TeamRole) -> Result<TeamRole, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamRole>, Box<dyn Error>>;

    async fn save(&self, team_role: TeamRole) -> Result<TeamRole, Box<dyn Error>>;
}
