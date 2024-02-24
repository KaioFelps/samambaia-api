use std::error::Error;

use async_trait::async_trait;
use sea_orm::ActiveModelTrait;

use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::infra::sea::mappers::sea_team_role_mapper::SeaTeamRoleMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaTeamRoleRepository {
    sea_service: SeaService
}

impl SeaTeamRoleRepository {
    // constructor
    pub async fn new(sea_service: SeaService) -> Self {
        SeaTeamRoleRepository {
            sea_service
        }
    }
}

#[async_trait]
impl TeamRoleRepositoryTrait for SeaTeamRoleRepository {
    async fn create(&self, team_role: TeamRole) -> Result<TeamRole, Box<dyn Error>> {
        let team_role = SeaTeamRoleMapper::team_role_to_sea_active_model(team_role);
        let team_role = team_role.insert(&self.sea_service.db).await?;
        let team_role = SeaTeamRoleMapper::model_to_team_role(team_role);

        Ok(team_role)
    }
}