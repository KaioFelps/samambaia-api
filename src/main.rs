use dotenvy::dotenv;
use env_logger::{self, Target};

use hubbitos_backend::domain::factories::create_team_role_service_factory;
use hubbitos_backend::domain::services::create_team_role_service::CreateTeamRoleParams;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use hubbitos_backend::infra::sea::sea_service::SeaService;
use hubbitos_backend::infra::sea::mappers::sea_user_mapper::SeaUserMapper;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();

    let floricultor_user = entities::user::Entity::find()
    .filter(entities::user::Column::Nickname.eq("Floricultor".to_owned()))
    .one(&SeaService::new().await.db)
    .await
    .unwrap()
    .unwrap();

    let _floricultor_user = SeaUserMapper::model_to_user(floricultor_user);

    let _ctrs = create_team_role_service_factory::exec().await;

    let res = _ctrs.exec(CreateTeamRoleParams {
        title: "Desenvolvedor".into(),
        description: "Desenvolver novas funcionalidades para o fã-site.".into(),
        staff_id: _floricultor_user.id()
    }).await;

    println!("{:?}", res.unwrap());
}
