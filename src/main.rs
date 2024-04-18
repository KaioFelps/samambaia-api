use actix_web::{middleware, App, HttpServer};
use dotenvy::dotenv;
use env_logger::{self, Target};
use hubbitos_backend::infra::http::routes::api::ApiRoutes;

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();

    let _floricultor_user = hubbitos_backend::infra::sea::mappers::sea_user_mapper::SeaUserMapper::model_to_user(
        entities::user::Entity::find()
        .filter(entities::user::Column::Nickname.eq("Floricultor".to_owned()))
        .one(&hubbitos_backend::infra::sea::sea_service::SeaService::new().await.db)
        .await
        .unwrap()
        .unwrap()
    );

    let token = hubbitos_backend::infra::jwt::jwt_service::JwtService{}.make_jwt(
        _floricultor_user.id(),
        _floricultor_user.role().unwrap(),
        jsonwebtoken::EncodingKey::from_secret(&hubbitos_backend::ENV_VARS.jwt_secret.as_ref())
    ).unwrap();

    println!("{:#?}", _floricultor_user);
    println!("{:#?}", token);

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
        .configure(ApiRoutes::register)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
