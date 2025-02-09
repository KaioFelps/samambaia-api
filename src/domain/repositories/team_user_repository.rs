use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::team_user::TeamUser;

#[derive(Debug)]
pub struct FindManyTeamUsersResponse(pub Vec<TeamUser>, pub u64);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TeamUserQueryType {
    TeamRole(Uuid),
    Nickname(String),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TeamUserRepositoryTrait {
    async fn create(&self, team_user: TeamUser) -> Result<TeamUser, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamUser>, Box<dyn Error>>;

    async fn save(&self, team_user: TeamUser) -> Result<TeamUser, Box<dyn Error>>;

    async fn delete(&self, team_role: TeamUser) -> Result<(), Box<dyn Error>>;

    async fn find_many(
        &self,
        params: PaginationParameters<TeamUserQueryType>,
    ) -> Result<FindManyTeamUsersResponse, Box<dyn Error>>;
}
