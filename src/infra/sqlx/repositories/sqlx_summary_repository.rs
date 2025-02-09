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
                (SELECT COUNT(id) FROM "user") AS "users!: i32",
                (SELECT COUNT(id) FROM "article") AS "articles!: i32",
                (SELECT COUNT(id) FROM "comment") AS "comments!: i32",
                (SELECT COUNT(id) FROM "user" where not "user"."role" = 'User') AS "team_users!: i32";"#
        )
        .fetch_one(self.sea_service.db.get_postgres_connection_pool())
        .await
        .map_err(Box::new)
        .map_err(|err| err as Box<dyn Error>)
    }
}
