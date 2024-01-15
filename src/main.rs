use entities::sea_orm_active_enums::Role;
use env_config::EnvConfig;
use factories::create_user_service_factory;
use infra::jwt::jwt_service::JwtService;
use jsonwebtoken::{EncodingKey, DecodingKey};
use once_cell::sync::Lazy;
use services::create_user_service::CreateUserParams;

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

    let jwt = JwtService::make_jwt(uuid::uuid!("a13196fd-c363-4be3-8ce4-e8d9fe648695"), EncodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));
    println!("Encoded jwt access token: {:#?}. Refresh token: {:#?}", &jwt.as_ref().unwrap().access_token, &jwt.as_ref().unwrap().refresh_token);

    let decoded_jwt = JwtService::decode_jwt(jwt.unwrap().access_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));
    println!("Decoded jwt token: {:#?}", decoded_jwt);
}
