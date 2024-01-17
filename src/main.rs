use entities::sea_orm_active_enums::Role;
use entities::user::Column;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use uuid::uuid;
use env_config::EnvConfig;
use once_cell::sync::Lazy;
use domain::factories::{authenticate_user_service_factory, update_user_service_factory, change_password_service_factory, create_user_service_factory};
use infra::sea::sea_service::SeaService;
use domain::services::authenticate_user_service::AuthenticateUserParams;
use domain::services::change_password_service::ChangePasswordParams;
use domain::services::update_user_service::UpdateUserParams;
use domain::services::create_user_service::CreateUserParams;

mod env_config;
mod infra;
mod errors;
mod util;

mod domain;


static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

#[tokio::main]
async fn main() { 
    let create_user_service = create_user_service_factory::exec().await;

    let _ = create_user_service.exec(CreateUserParams {
        nickname: "Fierce".to_string(),
        password: "velhalinda123".to_string()
    }).await;

    let _ = create_user_service.exec_with_custom_role(CreateUserParams {
        nickname: "Floricultor".to_string(),
        password: "123456".to_owned(),
    }, Role::Ceo).await;

    let authenticate_user_service = authenticate_user_service_factory::exec().await;

    let tokens = authenticate_user_service.exec(AuthenticateUserParams { nickname: "Floricultor".to_string(), password: "123456".to_string() }).await;

    println!("Encoded jwt access token: {:#?}. Refresh token: {:#?}", &tokens.as_ref().unwrap().access_token, &tokens.as_ref().unwrap().refresh_token);


    let update_user_service = update_user_service_factory::exec().await;

    let _res = update_user_service.exec(UpdateUserParams {
        nickname: None,
        password: Some("athos123".to_string()),
        role: Some(entities::sea_orm_active_enums::Role::Coord),
        staff_id: uuid!("a13196fd-c363-4be3-8ce4-e8d9fe648695"),
        user_id: uuid!("f7e38e5e-b0fd-4e28-b7a6-79a4bb38eb3c"),
    }).await;

    let floricultor_user = entities::user::Entity::find().filter(Column::Nickname.eq("Floricultor".to_owned())).one(&SeaService::new().await.db).await.unwrap().unwrap();
    let old_password = floricultor_user.password;

    let change_password_service = change_password_service_factory::exec().await;

    let _ = change_password_service.exec(ChangePasswordParams {
        user_id: floricultor_user.id,
        current_password: "123456".to_string(),
        new_password: "123456791023".to_string()
    }).await;

    let new_password = entities::user::Entity::find().filter(Column::Nickname.eq("Floricultor".to_owned())).one(&SeaService::new().await.db).await.unwrap().unwrap().password;

    assert_ne!(new_password, old_password);

    // let decoded_access_token = JwtService {}.decode_jwt(tokens.as_ref().unwrap().access_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));
    // let decoded_refresh_token = JwtService {}.decode_jwt(tokens.unwrap().refresh_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));

    // println!("DECODED ACCESS TOKEN: {:#?}. \nDECODED REFRESH TOKEN: {:#?}", decoded_access_token, decoded_refresh_token);
}
