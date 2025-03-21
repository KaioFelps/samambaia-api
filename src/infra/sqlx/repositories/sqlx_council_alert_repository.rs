use std::error::Error;

use async_trait::async_trait;

use crate::domain::domain_entities::council_alert::{CouncilAlert, CouncilAlertDraft};
use crate::domain::repositories::council_alerts_repository::CouncilAlertRepositoryTrait;
use crate::infra::sea::sea_service::SeaService;

pub struct SqlxCouncilAlertRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SqlxCouncilAlertRepository<'a> {
    pub fn new(sea_service: &'a SeaService) -> Self {
        Self { sea_service }
    }
}

#[async_trait]
impl CouncilAlertRepositoryTrait for SqlxCouncilAlertRepository<'_> {
    async fn create(
        &self,
        council_alert: CouncilAlertDraft,
    ) -> Result<CouncilAlert, Box<dyn Error>> {
        let (id,): (i32,) = sqlx::query_as(
            r#"INSERT INTO "council_alert"
            (title, content, pinned, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(council_alert.title())
        .bind(council_alert.content())
        .bind(council_alert.pinned())
        .bind(council_alert.created_at())
        .fetch_one(self.sea_service.db.get_postgres_connection_pool())
        .await
        .map_err(Box::new)?;

        Ok(council_alert.into_council_alert(id))
    }

    async fn find_all(&self) -> Result<Vec<CouncilAlert>, Box<dyn Error>> {
        sqlx::query_as(
            r#"SELECT id, pinned, title, content, created_at
            FROM "council_alert"
            ORDER BY
                pinned DESC,
                created_at DESC"#,
        )
        .fetch_all(self.sea_service.db.get_postgres_connection_pool())
        .await
        .map_err(Box::new)
        .map_err(|err| err as Box<dyn Error>)
    }
}
