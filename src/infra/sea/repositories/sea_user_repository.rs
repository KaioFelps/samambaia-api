use sea_orm::{EntityTrait, ActiveModelTrait, IntoActiveValue, DbErr, QueryFilter, ColumnTrait, ActiveValue};
use entities::user::Model as UserModel;
use entities::sea_orm_active_enums::Role as UserRole;
use crate::infra::sea::sea_service::SeaService;

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

impl SeaUserRepository {
    pub async fn create(&self, nickname: String, password: String, role: UserRole) -> Result<UserModel, DbErr> {
        use uuid::Uuid;

        let new_user = entities::user::ActiveModel {
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
        entities::user::Entity::find().all(&self.sea_service.db).await
    }

    pub async fn find_by_nickname(&self, nickname: &String) -> Result<std::option::Option<entities::user::Model>, sea_orm::DbErr> {
        entities::user::Entity::find().filter(entities::user::Column::Nickname.eq(nickname)).one(&self.sea_service.db).await
    }
}