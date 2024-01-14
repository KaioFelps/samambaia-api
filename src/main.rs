use entities::sea_orm_active_enums::Role;
use env_config::EnvConfig;
use factories::create_user_service_factory;
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
}
