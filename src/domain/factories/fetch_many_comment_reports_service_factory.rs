use crate::domain::services::fetch_many_comment_reports_service::FetchManyCommentReportsService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_comment_report_repository::SeaCommentReportRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchManyCommentReportsService<SeaCommentReportRepository, SeaUserRepository>, HttpResponse>{
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let comment_report_repository: Box<SeaCommentReportRepository> = Box::new(SeaCommentReportRepository::new(sea_service.clone()).await);
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);
    
    let fetch_many_comment_reports_service = FetchManyCommentReportsService::new(comment_report_repository, user_repository);

    Left(fetch_many_comment_reports_service)
}