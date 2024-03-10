use dotenvy::dotenv;
use env_logger::{self, Target};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use hubbitos_backend::infra::sea::sea_service::SeaService;
use hubbitos_backend::infra::sea::mappers::sea_user_mapper::SeaUserMapper;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();

    let _floricultor_user = SeaUserMapper::model_to_user(
            entities::user::Entity::find()
        .filter(entities::user::Column::Nickname.eq("Floricultor".to_owned()))
        .one(&SeaService::new().await.db)
        .await
        .unwrap()
        .unwrap()
    );
}
