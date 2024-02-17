use crate::infra::sea::sea_service::SeaService;
use crate::domain::services::toggle_comment_visibility_service::ToggleCommentVisibilityService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> ToggleCommentVisibilityService<SeaUserRepository, SeaCommentRepository> {
    let sea_service = SeaService::new().await;
    
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let comment_repository: Box<SeaCommentRepository> = Box::new(SeaCommentRepository::new(sea_service).await);
    
    let toggle_comment_visibility_service = ToggleCommentVisibilityService::new(
        user_repository,
        comment_repository
    );

    toggle_comment_visibility_service
}