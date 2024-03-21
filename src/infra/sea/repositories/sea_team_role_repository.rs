use std::error::Error;

use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait};
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::{FindManyTeamRolesResponse, TeamRoleQueryType, TeamRoleRepositoryTrait};
use crate::infra::sea::mappers::sea_team_role_mapper::SeaTeamRoleMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::team_role::Entity as TeamRoleEntity;
use entities::team_role::Column as TeamRoleColumn;

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

    async fn find_many(&self, params: PaginationParameters<TeamRoleQueryType>) -> Result<FindManyTeamRolesResponse, Box<dyn Error>> {
        let team_roles_response;

        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = ((&current_page - 1) * items_per_page) as u64;

        team_roles_response = TeamRoleEntity::find()
        .order_by_desc(TeamRoleColumn::CreatedAt)
        .apply_if(params.clone().query, |query_builder, query| self.find_many_get_filters(query_builder, query))
        .limit(items_per_page)
        .offset(leap)
        .all(&self.sea_service.db).await?;

        let team_roles_count = TeamRoleEntity::find()
        .apply_if(params.query, |query_builder, query| self.find_many_get_filters(query_builder, query))
        .offset(leap)
        .count(&self.sea_service.db).await?;

        let mut team_roles: Vec<TeamRole> = vec![];

        for team_role in team_roles_response.into_iter() {
            team_roles.push(SeaTeamRoleMapper::model_to_team_role(team_role));
        }

        Ok(FindManyTeamRolesResponse(team_roles, team_roles_count))
    }
}

impl SeaTeamRoleRepository {
    fn find_many_get_filters(&self, #[allow(unused_mut)] mut query_builder: sea_orm::Select<TeamRoleEntity>, query: TeamRoleQueryType) -> sea_orm::Select<TeamRoleEntity> {
        match query {
            TeamRoleQueryType::TITLE(content) => {
                let filter = Expr::expr(Func::lower(Expr::col(TeamRoleColumn::Title))).like(format!("%{}%", content.to_lowercase()));
                query_builder.filter(filter.clone())
            }
        }
    }
}