use crate::domain::services::fetch_many_comment_reports_service::FetchManyCommentReportsService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> FetchManyCommentReportsService<SeaCommentReportRepository, SeaUserRepository> {
    let comment_report_repository = SeaCommentReportRepository::new(db_conn);
    let user_repository = SeaUserRepository::new(db_conn);

    FetchManyCommentReportsService::new(comment_report_repository, user_repository)
}
