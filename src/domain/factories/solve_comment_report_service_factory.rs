use crate::domain::services::solve_comment_report_service::SolveCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> SolveCommentReportService<SeaCommentReportRepository> {
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(db_conn));
    SolveCommentReportService::new(comment_report_repository)
}
