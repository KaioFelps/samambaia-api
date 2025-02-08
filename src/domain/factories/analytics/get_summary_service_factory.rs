use crate::domain::services::analytics::get_summary_service::GetSummaryService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sqlx::repositories::sqlx_summary_repository::SqlxSummaryRepository;

pub fn exec(db_conn: &SeaService) -> GetSummaryService<SqlxSummaryRepository> {
    let sqlx_summary_repository = SqlxSummaryRepository::new(db_conn);
    GetSummaryService::new(sqlx_summary_repository)
}
