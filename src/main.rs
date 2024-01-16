use entities::sea_orm_active_enums::Role;
use env_config::EnvConfig;
use factories::create_user_service_factory;
use infra::jwt::jwt_service::JwtService;
use jsonwebtoken::DecodingKey;
use once_cell::sync::Lazy;
use services::create_user_service::CreateUserParams;

use crate::{factories::authenticate_user_service_factory, services::authenticate_user_service::AuthenticateUserParams};

mod env_config;
mod services;
mod infra;
mod errors;
mod factories;

static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

#[tokio::main]
async fn main() { 
    let create_user_service = create_user_service_factory::exec().await;

    let _result = create_user_service.exec_with_custom_role(CreateUserParams {
        nickname: "Floricultor".to_string(),
        password: "123456".to_string()
    }, Role::Principal).await;

    let authenticate_user_service = authenticate_user_service_factory::exec().await;

    let tokens = authenticate_user_service.exec(AuthenticateUserParams { nickname: "Floricultor".to_string(), password: "123456".to_string() }).await;

    println!("Encoded jwt access token: {:#?}. Refresh token: {:#?}", &tokens.as_ref().unwrap().access_token, &tokens.as_ref().unwrap().refresh_token);


    


    let decoded_access_token = JwtService {}.decode_jwt(tokens.as_ref().unwrap().access_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));
    let decoded_refresh_token = JwtService {}.decode_jwt(tokens.unwrap().refresh_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));

    println!("DECODED ACCESS TOKEN: {:#?}. \nDECODED REFRESH TOKEN: {:#?}", decoded_access_token, decoded_refresh_token);
}
