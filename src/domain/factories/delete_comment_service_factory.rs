use crate::domain::services::delete_comment_service::DeleteCommentService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<DeleteCommentService<SeaCommentRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let comment_repository: Box<SeaCommentRepository> =
        Box::new(SeaCommentRepository::new(sea_service).await);

    let delete_comment_service = DeleteCommentService::new(comment_repository);

    Ok(delete_comment_service)
}
