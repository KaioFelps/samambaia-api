use crate::domain::services::solve_comment_report_service::SolveCommentReportService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<SolveCommentReportService<SeaCommentReportRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let comment_report_repository: Box<SeaCommentReportRepository> =
        Box::new(SeaCommentReportRepository::new(sea_service).await);

    let solve_comment_report_service = SolveCommentReportService::new(comment_report_repository);

    Ok(solve_comment_report_service)
}
