use crate::domain::services::create_comment_report_service::CreateCommentReportService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<CreateCommentReportService<SeaCommentRepository, SeaCommentReportRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    
    let comment_repository = Box::new(SeaCommentRepository::new(sea_service.clone()).await);
    let comment_report_repository = Box::new(SeaCommentReportRepository::new(sea_service).await);
    
    let create_comment_report_service = CreateCommentReportService::new(
        comment_repository,
        comment_report_repository
    );

    Left(create_comment_report_service)
}