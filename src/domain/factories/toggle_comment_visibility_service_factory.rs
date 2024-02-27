use crate::infra::sea::sea_service::SeaService;
use crate::domain::services::toggle_comment_visibility_service::ToggleCommentVisibilityService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;

pub async fn exec() -> ToggleCommentVisibilityService<SeaCommentRepository> {
    let sea_service = SeaService::new().await;
    
    let comment_repository: Box<SeaCommentRepository> = Box::new(SeaCommentRepository::new(sea_service).await);
    
    let toggle_comment_visibility_service = ToggleCommentVisibilityService::new(
        comment_repository
    );

    toggle_comment_visibility_service
}