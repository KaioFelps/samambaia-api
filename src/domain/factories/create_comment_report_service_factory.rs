use crate::domain::services::create_comment_report_service::CreateCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> CreateCommentReportService<SeaCommentRepository, SeaCommentReportRepository> {
    let comment_repository = SeaCommentRepository::new(db_conn);
    let comment_report_repository = SeaCommentReportRepository::new(db_conn);

    CreateCommentReportService::new(comment_repository, comment_report_repository)
}
