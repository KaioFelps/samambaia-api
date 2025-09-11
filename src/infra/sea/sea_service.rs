use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use crate::configs::app::APP_CONFIG;
use crate::{LOG_SEP, R_EOL};

// #[derive(Clone)]
pub struct SeaService {
    pub db: DatabaseConnection,
}

async fn get_db_conn() -> Result<DatabaseConnection, DbErr> {
    let mut db_opts: ConnectOptions = ConnectOptions::new(APP_CONFIG.database_url);
    db_opts
        .max_connections(15)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let connection = Database::connect(db_opts).await;

    match connection {
        Err(err) => {
            log::error!("{R_EOL}{LOG_SEP}{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}", err);
            Err(err)
        }
        Ok(connection) => Ok(connection),
    }
}

impl SeaService {
    pub async fn new() -> Result<Self, DbErr> {
        let db = get_db_conn().await?;

        Ok(Self { db })
    }
}

impl Clone for SeaService {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}
