use crate::domain::services::create_comment_report_service::CreateCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> CreateCommentReportService<SeaCommentRepository, SeaCommentReportRepository> {
    let comment_repository = Box::new(SeaCommentRepository::new(db_conn).await);
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(db_conn).await);

    CreateCommentReportService::new(comment_repository, comment_report_repository)
}
