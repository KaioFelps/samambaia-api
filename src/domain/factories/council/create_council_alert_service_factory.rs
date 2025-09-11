use crate::domain::services::council::create_council_alert_service::CreateCouncilAlertService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sqlx::repositories::sqlx_council_alert_repository::SqlxCouncilAlertRepository;

pub fn exec(db_conn: &SeaService) -> CreateCouncilAlertService<SqlxCouncilAlertRepository<'_>> {
    let council_alert_repository = SqlxCouncilAlertRepository::new(db_conn);
    CreateCouncilAlertService::new(council_alert_repository)
}
