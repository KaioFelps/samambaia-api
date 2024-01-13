use sea_orm::{EntityTrait, ActiveModelTrait, IntoActiveValue};

use crate::{repositories::user_repository::{UserRepository, CreateUserParam}, infra::sea::sea_service::SeaService};

pub struct SeaUserRepository {
    // db_conn: DatabaseConnection,
    sea_service: SeaService
}

impl SeaUserRepository {
    // constructor
    pub async fn new(service: &SeaService) -> Self {
        SeaUserRepository {
            sea_service: service.clone(),
        }
    }
}

impl UserRepository for SeaUserRepository {
    async fn create(&self, params: CreateUserParam) -> Result<entities::user::Model, sea_orm::DbErr> {
        use uuid::Uuid;

        let new_user = entities::user::ActiveModel {
            id: Uuid::new_v4().into_active_value(),
            nickname: params.nickname.into_active_value(),
            password: params.password().into_active_value(),
            ..Default::default()
        };

        let created_user = new_user.insert(&self.sea_service.db).await.unwrap();
        Ok(created_user)
    }

    async fn find_all(&self) -> Result<Vec<entities::user::Model>, sea_orm::DbErr> {
        entities::user::Entity::find().all(&self.sea_service.db).await
    }
}