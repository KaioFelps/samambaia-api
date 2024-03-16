use std::error::Error;

use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::domain::domain_entities::team_user::TeamUser;
use crate::domain::repositories::team_user_repository::TeamUserRepositoryTrait;
use crate::infra::sea::mappers::sea_team_user_mapper::SeaTeamUserMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::team_user::Entity as TeamUserEntity;

pub struct SeaTeamUserRepository {
    sea_service: SeaService
}

impl SeaTeamUserRepository {
    // constructor
    pub async fn new(sea_service: SeaService) -> Self {
        SeaTeamUserRepository {
            sea_service
        }
    }
}

#[async_trait]
impl TeamUserRepositoryTrait for SeaTeamUserRepository {
    async fn create(&self, team_user: TeamUser) -> Result<TeamUser, Box<dyn Error>> {
        let team_user = SeaTeamUserMapper::team_user_to_sea_active_model(team_user);
        let team_user = team_user.insert(&self.sea_service.db).await?;
        let team_user = SeaTeamUserMapper::model_to_team_user(team_user);

        Ok(team_user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamUser>, Box<dyn Error>> {
        let team_user = TeamUserEntity::find_by_id(id).one(&self.sea_service.db).await?;

        match team_user {
            None => Ok(None),
            Some(team_user) => {
                let mapped_team_user = SeaTeamUserMapper::model_to_team_user(team_user);
                Ok(Some(mapped_team_user))
            }
        }
    }

    async fn save(&self, team_user: TeamUser) -> Result<TeamUser, Box<dyn Error>> {
        let team_user = SeaTeamUserMapper::team_user_to_sea_active_model(team_user);

        let team_user = team_user.update(&self.sea_service.db).await?;

        Ok(SeaTeamUserMapper::model_to_team_user(team_user))
    }
}