use crate::domain::services::create_comment_report_service::CreateCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> CreateCommentReportService<SeaCommentRepository, SeaCommentReportRepository> {
    let sea_service = SeaService::new().await;
    
    let comment_repository = Box::new(SeaCommentRepository::new(sea_service.clone()).await);
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(sea_service).await);
    
    let create_comment_report_service = CreateCommentReportService::new(
        comment_repository,
        comment_report_repository
    );

    create_comment_report_service
}