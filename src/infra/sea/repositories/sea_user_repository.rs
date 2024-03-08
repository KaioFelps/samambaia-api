use std::error::Error;

use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{EntityTrait, ActiveModelTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use crate::infra::sea::sea_service::SeaService;
use crate::domain::domain_entities::user::User;
use crate::infra::sea::mappers::sea_user_mapper::SeaUserMapper;
use entities::user::{Column as UserColumn, Entity as UserEntity};
use crate::domain::repositories::user_repository::UserRepositoryTrait;

pub struct SeaUserRepository {
    pub sea_service: SeaService,
}

impl SeaUserRepository {
    pub async fn new(sea_service: SeaService) -> SeaUserRepository {

        SeaUserRepository {
            sea_service,
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for SeaUserRepository {
    async fn create(&self, user: User) -> Result<User, Box<dyn Error>> {
        let new_user = SeaUserMapper::user_to_sea_active_model(user);

        let db = &self.sea_service.db;

        let created_user = new_user.insert(db).await?;
        let created_user = SeaUserMapper::model_to_user(created_user);

        Ok(created_user)
    }

    async fn find_by_nickname(&self, nickname: &String) -> Result<Option<User>, Box<dyn Error>> {
        let user = UserEntity::find()
        .filter(Expr::expr(Func::lower(Expr::col(UserColumn::Nickname))).like(nickname))
        .one(&self.sea_service.db)
        .await?;

        if user.is_none() {
            return Ok(None);
        }

        return Ok(Some(SeaUserMapper::model_to_user(user.unwrap())));
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let user = UserEntity::find_by_id(*id)
        .one(&self.sea_service.db)
        .await?;

        if user.is_none() {
            return Ok(None);
        }

        return Ok(Some(SeaUserMapper::model_to_user(user.unwrap())));
    }

    async fn save(&self, user: User) -> Result<User, Box<dyn Error>> {
        let user_id = &user.id().clone();

        let user = SeaUserMapper::user_to_sea_active_model(user.clone());

        let user = UserEntity::update(user.clone())
        .filter(UserColumn::Id.eq(*user_id))
        .exec(&self.sea_service.db)
        .await?;

        let user = SeaUserMapper::model_to_user(user);

        Ok(user)
    }
}