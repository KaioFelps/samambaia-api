use crate::domain::services::solve_comment_report_service::SolveCommentReportService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<SolveCommentReportService<SeaCommentReportRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let comment_report_repository: Box<SeaCommentReportRepository> =
        Box::new(SeaCommentReportRepository::new(sea_service).await);

    let solve_comment_report_service = SolveCommentReportService::new(comment_report_repository);

    Left(solve_comment_report_service)
}
