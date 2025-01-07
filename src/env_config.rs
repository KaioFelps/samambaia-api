use core::panic;

use dotenvy::dotenv;
use serde::Deserialize;
use serde_envfile::from_env as lib_from_env;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RustEnv {
    Production,
    Development,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub rust_env: RustEnv,
    pub database_url: String,
    pub domain: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub https: bool,
    pub workers: usize,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let env: Result<Self, serde_envfile::Error> = lib_from_env();

        match env {
            Err(error) => panic!("Invalid environment variables: {:#?}", error),
            Ok(value) => value,
        }
    }
}
