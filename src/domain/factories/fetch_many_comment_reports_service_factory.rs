use crate::domain::services::fetch_many_comment_reports_service::FetchManyCommentReportsService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> FetchManyCommentReportsService<SeaCommentReportRepository> {
    let sea_service = SeaService::new().await;

    let comment_report_repository: Box<SeaCommentReportRepository> = Box::new(SeaCommentReportRepository::new(sea_service).await);
    
    let fetch_many_comment_reports_service = FetchManyCommentReportsService::new(comment_report_repository);

    fetch_many_comment_reports_service
}