use env_config::EnvConfig;
use once_cell::sync::Lazy;

pub mod core;
pub mod domain;
pub mod env_config;
pub mod error;
pub mod infra;
pub mod libs;
pub mod server;
mod tests;
pub mod util;

pub static ENV_VARS: Lazy<EnvConfig> = Lazy::new(EnvConfig::from_env);

#[cfg(target_os = "windows")]
const R_EOL: &str = "\r\n";

#[cfg(not(target_os = "windows"))]
const R_EOL: &'static str = "\n";

const LOG_SEP: &str = "==============";

pub use async_trait;
pub use tokio;
pub use uuid;
