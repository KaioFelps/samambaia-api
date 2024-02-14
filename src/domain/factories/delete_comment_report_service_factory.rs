use crate::domain::services::delete_comment_report_service::DeleteCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> DeleteCommentReportService<SeaUserRepository, SeaCommentReportRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let comment_report_repository: Box<SeaCommentReportRepository> = Box::new(SeaCommentReportRepository::new(sea_service).await);
    
    let delete_comment_report_service = DeleteCommentReportService::new(
        user_repository,
        comment_report_repository
    );

    delete_comment_report_service
}