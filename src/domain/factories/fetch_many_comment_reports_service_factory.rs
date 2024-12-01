use crate::domain::services::fetch_many_comment_reports_service::FetchManyCommentReportsService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> FetchManyCommentReportsService<SeaCommentReportRepository, SeaUserRepository> {
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(db_conn).await);
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);

    FetchManyCommentReportsService::new(comment_report_repository, user_repository)
}
