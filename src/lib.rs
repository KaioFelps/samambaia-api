use env_config::EnvConfig;
use once_cell::sync::Lazy;

pub mod env_config;
pub mod infra;
pub mod errors;
pub mod util;
pub mod domain;

static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

pub use domain::repositories::user_repository::UserRepositoryTrait;
pub use uuid;