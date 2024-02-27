use crate::domain::services::delete_comment_report_service::DeleteCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> DeleteCommentReportService<SeaCommentReportRepository> {
    let sea_service = SeaService::new().await;

    let comment_report_repository: Box<SeaCommentReportRepository> = Box::new(SeaCommentReportRepository::new(sea_service).await);
    
    let delete_comment_report_service = DeleteCommentReportService::new(
        comment_report_repository
    );

    delete_comment_report_service
}