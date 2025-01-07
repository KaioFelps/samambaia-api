use crate::{ENV_VARS, LOG_SEP, R_EOL};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

// #[derive(Clone)]
pub struct SeaService {
    pub db: DatabaseConnection,
}

async fn get_db_conn() -> Result<DatabaseConnection, DbErr> {
    let mut db_opts: ConnectOptions = ConnectOptions::new(&ENV_VARS.database_url);
    db_opts
        .max_connections(15)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let connection = Database::connect(db_opts).await;

    if connection.is_err() {
        let err = connection.unwrap_err();

        log::error!(
            "{R_EOL}{LOG_SEP}{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
            err.to_string()
        );

        return Err(err);
    }

    Ok(connection.unwrap())
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
