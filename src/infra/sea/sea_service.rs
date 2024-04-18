use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::ENV_VARS;

// #[derive(Clone)]
pub struct SeaService {
    pub db: DatabaseConnection
}

async fn get_db_conn() -> DatabaseConnection {
    let mut db_opts: ConnectOptions = ConnectOptions::new(&ENV_VARS.database_url);
    db_opts.max_connections(15)
    .connect_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(true)
    .set_schema_search_path("public");

    Database::connect(db_opts).await.expect("Database connection failed.")
}

impl SeaService {
    pub async fn new () -> Self {
        Self {
            db: get_db_conn().await
        }
    }
}

impl Clone for SeaService {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone()
        }
    }
}