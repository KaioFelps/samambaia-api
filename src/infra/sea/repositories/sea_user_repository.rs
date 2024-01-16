use sea_orm::{EntityTrait, ActiveModelTrait, IntoActiveValue, DbErr, QueryFilter, ColumnTrait, ActiveValue};
use entities::sea_orm_active_enums::Role as UserRole;
use uuid::Uuid;
use crate::infra::sea::sea_service::SeaService;
use entities::user::{Model as UserModel, Column as UserColumn, Entity as UserEntity, ActiveModel as UserActiveModel};

pub struct SeaUserRepository {
    pub sea_service: SeaService,
}

impl SeaUserRepository {
    pub async fn new(sea_service: SeaService) -> SeaUserRepository {

        SeaUserRepository {
            sea_service,
        }
    }

    pub async fn create(&self, nickname: String, password: String, role: UserRole) -> Result<UserModel, DbErr> {

        let new_user = UserActiveModel {
            id: Uuid::new_v4().into_active_value(),
            nickname: nickname.into_active_value(),
            password: password.into_active_value(),
            role: ActiveValue::Set(Some(role)),
            ..Default::default()
        };

        let db = &self.sea_service.db;

        let created_user = new_user.insert(db).await.unwrap();
        Ok(created_user)
    }

    pub async fn find_all(&self) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        UserEntity::find().all(&self.sea_service.db).await
    }

    pub async fn find_by_nickname(&self, nickname: &String) -> Result<Option<UserModel>, DbErr> {
        UserEntity::find().filter(UserColumn::Nickname.eq(nickname)).one(&self.sea_service.db).await
    }
}