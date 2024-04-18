use env_config::EnvConfig;
use once_cell::sync::Lazy;

pub mod env_config;
pub mod infra;
pub mod errors;
pub mod util;
pub mod domain;
pub mod core;

pub static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

#[cfg(target_os = "windows")]
const R_EOL: &'static str = "\r\n"; 

#[cfg(not(target_os = "windows"))]
const R_EOL: &'static str = "\n"; 

const LOG_SEP: &'static str = "==============";

pub use uuid;
pub use tokio;
pub use async_trait;