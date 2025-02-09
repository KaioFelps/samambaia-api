use std::error::Error;

use async_trait::async_trait;
use sea_orm::sqlx;

use crate::domain::repositories::summary_repository::SummaryRepositoryTrait;
use crate::domain::value_objects::count_summary::CountSummary;
use crate::infra::sea::sea_service::SeaService;

pub struct SqlxSummaryRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SqlxSummaryRepository<'a> {
    // constructor
    pub fn new(service: &'a SeaService) -> Self {
        SqlxSummaryRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl SummaryRepositoryTrait for SqlxSummaryRepository<'_> {
    async fn get_table_summary(&self) -> Result<CountSummary, Box<dyn Error>> {
        sqlx::query_as(
            r#"SELECT
                (SELECT COUNT(id)::Int4 FROM "user") AS users,
                (SELECT COUNT(id)::Int4 FROM "article") AS articles,
                (SELECT COUNT(id)::Int4 FROM "comment") AS comments,
                (SELECT COUNT(id)::Int4 FROM "user" WHERE NOT "user"."role" = 'User') AS team_users;"#,
        )
        .fetch_one(self.sea_service.db.get_postgres_connection_pool())
        .await
        .map_err(Box::new)
        .map_err(|err| err as Box<dyn Error>)
    }
}
