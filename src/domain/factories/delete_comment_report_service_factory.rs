use crate::domain::services::delete_comment_report_service::DeleteCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> DeleteCommentReportService<SeaCommentReportRepository> {
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(db_conn).await);
    DeleteCommentReportService::new(comment_report_repository)
}
