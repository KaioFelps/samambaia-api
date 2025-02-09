use std::error::Error;

use async_trait::async_trait;
use entities::user::{Column as UserColumn, Entity as UserEntity};
use migration::{Alias, Expr, Func};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    IntoSimpleExpr,
    PaginatorTrait,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    QueryTrait,
};
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::{
    FindManyUsersResponse,
    UserQueryType,
    UserRepositoryTrait,
};
use crate::infra::sea::mappers::sea_role_mapper::SeaRoleMapper;
use crate::infra::sea::mappers::sea_user_mapper::SeaUserMapper;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaUserRepository<'a> {
    pub sea_service: &'a SeaService,
}

impl SeaUserRepository<'_> {
    pub fn new(sea_service: &SeaService) -> SeaUserRepository<'_> {
        SeaUserRepository { sea_service }
    }
}

#[async_trait]
impl UserRepositoryTrait for SeaUserRepository<'_> {
    async fn create(&self, user: User) -> Result<User, Box<dyn Error>> {
        let new_user = SeaUserMapper::entity_into_active_model(user);

        let db = &self.sea_service.db;

        let created_user = new_user.insert(db).await?;
        let created_user = SeaUserMapper::model_into_entity(created_user);

        Ok(created_user)
    }

    async fn find_by_nickname(&self, nickname: &str) -> Result<Option<User>, Box<dyn Error>> {
        let user = UserEntity::find()
            .filter(
                Expr::expr(Func::lower(Expr::col(UserColumn::Nickname)))
                    .like(nickname.to_lowercase()),
            )
            .one(&self.sea_service.db)
            .await?;

        if user.is_none() {
            return Ok(None);
        }

        return Ok(Some(SeaUserMapper::model_into_entity(user.unwrap())));
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let user = UserEntity::find_by_id(*id)
            .one(&self.sea_service.db)
            .await?;

        if user.is_none() {
            return Ok(None);
        }

        return Ok(Some(SeaUserMapper::model_into_entity(user.unwrap())));
    }

    async fn save(&self, user: User) -> Result<User, Box<dyn Error>> {
        let user_id = user.id();

        let user = SeaUserMapper::entity_into_active_model(user.clone());

        let user = UserEntity::update(user.clone())
            .filter(UserColumn::Id.eq(user_id))
            .exec(&self.sea_service.db)
            .await?;

        let user = SeaUserMapper::model_into_entity(user);

        Ok(user)
    }

    async fn find_many(
        &self,
        params: PaginationParameters<UserQueryType>,
    ) -> Result<FindManyUsersResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let users_response = UserEntity::find()
            .order_by_desc(UserColumn::CreatedAt)
            .apply_if(params.clone().query, |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let users_count = UserEntity::find()
            .apply_if(params.query, |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mut users: Vec<User> = vec![];

        for role in users_response.into_iter() {
            users.push(SeaUserMapper::model_into_entity(role));
        }

        Ok(FindManyUsersResponse(users, users_count))
    }
}

impl SeaUserRepository<'_> {
    fn find_many_get_filters(
        &self,
        #[allow(unused_mut)] mut query_builder: sea_orm::Select<UserEntity>,
        query: UserQueryType,
    ) -> sea_orm::Select<UserEntity> {
        match query {
            UserQueryType::Nickname(content) => {
                let filter = Expr::expr(Func::lower(Expr::col(UserColumn::Nickname)))
                    .like(format!("%{}%", content.to_lowercase()));
                query_builder.filter(filter)
            }
            UserQueryType::Role(content) => {
                let filter = Expr::expr(Func::cast_as(
                    UserColumn::Role.into_simple_expr(),
                    Alias::new("text"),
                ))
                .eq(SeaRoleMapper::into_model(content));
                query_builder.filter(filter)
            }
        }
    }
}
