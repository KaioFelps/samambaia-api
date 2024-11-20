use crate::domain::services::delete_comment_service::DeleteCommentService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<DeleteCommentService<SeaCommentRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let comment_repository: Box<SeaCommentRepository> =
        Box::new(SeaCommentRepository::new(sea_service).await);

    let delete_comment_service = DeleteCommentService::new(comment_repository);

    Left(delete_comment_service)
}
