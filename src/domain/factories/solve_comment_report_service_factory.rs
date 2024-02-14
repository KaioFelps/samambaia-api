use crate::domain::services::solve_comment_report_service::SolveCommentReportService;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> SolveCommentReportService<SeaUserRepository, SeaCommentReportRepository> {
    let sea_service = SeaService::new().await;
    
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let comment_report_repository: Box<SeaCommentReportRepository> = Box::new(SeaCommentReportRepository::new(sea_service.clone()).await);
    
    let solve_comment_report_service = SolveCommentReportService::new(
        user_repository,
        comment_report_repository,
    );

    solve_comment_report_service
}