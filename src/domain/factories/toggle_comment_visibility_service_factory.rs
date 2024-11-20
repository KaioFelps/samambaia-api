use crate::domain::services::toggle_comment_visibility_service::ToggleCommentVisibilityService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<ToggleCommentVisibilityService<SeaCommentRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let comment_repository: Box<SeaCommentRepository> =
        Box::new(SeaCommentRepository::new(sea_service).await);

    let toggle_comment_visibility_service = ToggleCommentVisibilityService::new(comment_repository);

    Ok(toggle_comment_visibility_service)
}
