use std::error::Error;

use async_trait::async_trait;
use uuid::Uuid;

use migration::Expr;
use migration::Func;
use sea_orm::{
    ActiveModelTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::team_user::TeamUser;
use crate::domain::repositories::team_user_repository::FindManyTeamUsersResponse;
use crate::domain::repositories::team_user_repository::TeamUserQueryType;
use crate::domain::repositories::team_user_repository::TeamUserRepositoryTrait;
use crate::infra::sea::mappers::sea_team_user_mapper::SeaTeamUserMapper;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::team_user::Column as TeamUserColumn;
use entities::team_user::Entity as TeamUserEntity;

pub struct SeaTeamUserRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SeaTeamUserRepository<'a> {
    // constructor
    pub fn new(sea_service: &'a SeaService) -> Self {
        SeaTeamUserRepository { sea_service }
    }
}

#[async_trait]
impl TeamUserRepositoryTrait for SeaTeamUserRepository<'_> {
    async fn create(&self, team_user: TeamUser) -> Result<TeamUser, Box<dyn Error>> {
        let team_user = SeaTeamUserMapper::entity_into_active_model(team_user);
        let team_user = team_user.insert(&self.sea_service.db).await?;
        let team_user = SeaTeamUserMapper::model_into_entity(team_user);

        Ok(team_user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamUser>, Box<dyn Error>> {
        let team_user = TeamUserEntity::find_by_id(id)
            .one(&self.sea_service.db)
            .await?;

        match team_user {
            None => Ok(None),
            Some(team_user) => {
                let mapped_team_user = SeaTeamUserMapper::model_into_entity(team_user);
                Ok(Some(mapped_team_user))
            }
        }
    }

    async fn save(&self, team_user: TeamUser) -> Result<TeamUser, Box<dyn Error>> {
        let team_user = SeaTeamUserMapper::entity_into_active_model(team_user);

        let team_user = team_user.update(&self.sea_service.db).await?;

        Ok(SeaTeamUserMapper::model_into_entity(team_user))
    }

    async fn delete(&self, team_user: TeamUser) -> Result<(), Box<dyn Error>> {
        let team_user = SeaTeamUserMapper::entity_into_active_model(team_user);

        team_user.delete(&self.sea_service.db).await?;

        Ok(())
    }

    async fn find_many(
        &self,
        params: PaginationParameters<TeamUserQueryType>,
    ) -> Result<FindManyTeamUsersResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let team_users_response = TeamUserEntity::find()
            .order_by_desc(TeamUserColumn::CreatedAt)
            .apply_if(params.clone().query, |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let team_users_count = TeamUserEntity::find()
            .apply_if(params.query, |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mut team_users: Vec<TeamUser> = vec![];

        for team_role in team_users_response.into_iter() {
            team_users.push(SeaTeamUserMapper::model_into_entity(team_role));
        }

        Ok(FindManyTeamUsersResponse(team_users, team_users_count))
    }
}

impl SeaTeamUserRepository<'_> {
    fn find_many_get_filters(
        &self,
        #[allow(unused_mut)] mut query_builder: sea_orm::Select<TeamUserEntity>,
        query: TeamUserQueryType,
    ) -> sea_orm::Select<TeamUserEntity> {
        match query {
            TeamUserQueryType::Nickname(content) => {
                let filter = Expr::expr(Func::lower(Expr::col(TeamUserColumn::Nickname)))
                    .like(format!("%{}%", content.to_lowercase()));
                query_builder.filter(filter)
            }
            TeamUserQueryType::TeamRole(content) => {
                let filter = Expr::col(TeamUserColumn::TeamRoleId).eq(content);
                query_builder.filter(filter)
            }
        }
    }
}
