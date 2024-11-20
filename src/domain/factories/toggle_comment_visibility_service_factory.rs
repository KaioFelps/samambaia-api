use crate::domain::services::toggle_comment_visibility_service::ToggleCommentVisibilityService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<ToggleCommentVisibilityService<SeaCommentRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let comment_repository: Box<SeaCommentRepository> =
        Box::new(SeaCommentRepository::new(sea_service).await);

    let toggle_comment_visibility_service = ToggleCommentVisibilityService::new(comment_repository);

    Left(toggle_comment_visibility_service)
}
