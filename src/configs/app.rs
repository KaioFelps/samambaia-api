use std::sync::LazyLock;

use super::env::{EnvConfig, RustEnv};

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(AppConfig::initialize);

pub struct AppConfig<'a> {
    // Whether the application is running on PRODUCTION or DEVELOPMENT mode
    pub rust_env: RustEnv,

    // The URL string of the database connection
    pub database_url: &'a str,

    // The canonical domain of the application
    pub domain: &'a str,

    // A secret using for signing JWT tokens
    pub jwt_secret: &'a str,

    // A key used for encrypting data
    pub app_key: &'a str,

    // The host where the server will run
    pub host: &'a str,

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

    // The directory where sessions will be stored at
    pub sessions_dir: &'a str,

    // The key that stores the expiration date inside the session json
    pub sessions_exp_key: &'a str,

    // The name of the cookie which holds the user's session id
    pub session_cookie: &'a str,
}

impl AppConfig<'_> {
    pub fn initialize() -> Self {
        let env_vars = EnvConfig::from_env();

        Self {
            lottery: [2, 100],
            sessions_dir: "storage/sessions",
            sessions_exp_key: "___expires_at___",
            session_cookie: "live_cosmic_session",

            // FROM ENVIRONMENT VARIABLES
            app_key: Box::leak(env_vars.app_key.into_boxed_str()),
            database_url: Box::leak(env_vars.database_url.into_boxed_str()),
            domain: Box::leak(env_vars.domain.into_boxed_str()),
            host: Box::leak(env_vars.host.into_boxed_str()),
            https: env_vars.https,
            jwt_secret: Box::leak(env_vars.jwt_secret.into_boxed_str()),
            port: env_vars.port,
            rust_env: env_vars.rust_env,
            workers: env_vars.workers,
        }
    }
}
