use std::error::Error;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait};
use uuid::Uuid;

use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::infra::sea::mappers::sea_team_role_mapper::SeaTeamRoleMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::team_role::Entity as TeamRoleEntity;

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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamRole>, Box<dyn Error>> {
        let team_role = TeamRoleEntity::find_by_id(id).one(&self.sea_service.db).await?;

        match team_role {
            None => Ok(None),
            Some(team_role) => {
                let mapped_team_role = SeaTeamRoleMapper::model_to_team_role(team_role);
                Ok(Some(mapped_team_role))
            }
        }
    }

    async fn save(&self, team_role: TeamRole) -> Result<TeamRole, Box<dyn Error>> {
        let team_role = SeaTeamRoleMapper::team_role_to_sea_active_model(team_role);

        let team_role = team_role.update(&self.sea_service.db).await?;

        Ok(SeaTeamRoleMapper::model_to_team_role(team_role))
    }

    async fn delete(&self, team_role: TeamRole) -> Result<(), Box<dyn Error>> {
        let team_role = SeaTeamRoleMapper::team_role_to_sea_active_model(team_role);

        team_role.delete(&self.sea_service.db).await?;

        Ok(())
    }
}