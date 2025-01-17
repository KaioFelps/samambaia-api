use std::sync::LazyLock;

use super::env::{EnvConfig, RustEnv};

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(AppConfig::initialize);

pub struct AppConfig {
    // Whether the application is running on PRODUCTION or DEVELOPMENT mode
    pub rust_env: RustEnv,

    // The URL string of the database connection
    pub database_url: String,

    // The canonical domain of the application
    pub domain: String,

    // A secret using for signing JWT tokens
    pub jwt_secret: String,

    // A key used for encrypting data
    pub app_key: String,

    // The host where the server will run
    pub host: String,

    // The port which the server will be listening to
    pub port: u16,

    // If true, https protocol is used. Otherwise, http is used.
    pub https: bool,

    // The amount of workers that must be used by the actix server
    pub workers: usize,

    // Just like Laravel, this is used to raffle a request to run the
    // garbage collector, which will remove unused resources
    // (such as expired sessions files)
    pub lottery: [u8; 2],
}

impl AppConfig {
    pub fn initialize() -> Self {
        let env_vars = EnvConfig::from_env();

        Self {
            app_key: env_vars.app_key,
            database_url: env_vars.database_url,
            domain: env_vars.domain,
            host: env_vars.host,
            https: env_vars.https,
            jwt_secret: env_vars.jwt_secret,
            port: env_vars.port,
            rust_env: env_vars.rust_env,
            workers: env_vars.workers,
            lottery: [2, 100],
        }
    }
}
