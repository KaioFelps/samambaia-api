use crate::domain::services::create_comment_report_service::CreateCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> CreateCommentReportService<SeaUserRepository, SeaCommentRepository, SeaCommentReportRepository> {
    let sea_service = SeaService::new().await;
    
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let comment_repository = Box::new(SeaCommentRepository::new(sea_service.clone()).await);
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(sea_service.clone()).await);
    
    let create_comment_report_service = CreateCommentReportService::new(
        user_repository,
        comment_repository,
        comment_report_repository
    );

    create_comment_report_service
}