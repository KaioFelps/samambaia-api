mod repositories;

use entities::sea_orm_active_enums::Role;
use uuid::uuid;
use env_config::EnvConfig;
use factories::create_user_service_factory;
// use infra::jwt::jwt_service::JwtService;
// use jsonwebtoken::DecodingKey;
use once_cell::sync::Lazy;
use services::create_user_service::CreateUserParams;
// use uuid::uuid;

use crate::factories::{authenticate_user_service_factory, update_user_service_factory};
use crate::services::authenticate_user_service::AuthenticateUserParams;
use crate::services::update_user_service::UpdateUserParams;

mod env_config;
mod infra;
mod errors;
mod util;
mod services;
mod factories;



static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

#[tokio::main]
async fn main() { 
    let create_user_service = create_user_service_factory::exec().await;

    let _ = create_user_service.exec_with_custom_role(CreateUserParams {
        nickname: "Fierce".to_string(),
        password: "velhalinda123".to_string()
    }, Role::Coord).await;

    let authenticate_user_service = authenticate_user_service_factory::exec().await;

    let tokens = authenticate_user_service.exec(AuthenticateUserParams { nickname: "Floricultor".to_string(), password: "123456".to_string() }).await;

    println!("Encoded jwt access token: {:#?}. Refresh token: {:#?}", &tokens.as_ref().unwrap().access_token, &tokens.as_ref().unwrap().refresh_token);


    let _ = create_user_service.exec(CreateUserParams {
        nickname: "Vamp".to_string(),
        password: "athos123".to_owned()
    }).await;

    let update_user_service = update_user_service_factory::exec().await;

    let res = update_user_service.exec(UpdateUserParams {
        nickname: None,
        password: Some("athos123".to_string()),
        role: Some(entities::sea_orm_active_enums::Role::Coord),
        staff_id: uuid!("a13196fd-c363-4be3-8ce4-e8d9fe648695"),
        user_id: uuid!("f7e38e5e-b0fd-4e28-b7a6-79a4bb38eb3c"),
    }).await;

    println!("deu certo mudar o cargo maior {:#?}", res.unwrap());

    // let decoded_access_token = JwtService {}.decode_jwt(tokens.as_ref().unwrap().access_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));
    // let decoded_refresh_token = JwtService {}.decode_jwt(tokens.unwrap().refresh_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));

    // println!("DECODED ACCESS TOKEN: {:#?}. \nDECODED REFRESH TOKEN: {:#?}", decoded_access_token, decoded_refresh_token);
}
