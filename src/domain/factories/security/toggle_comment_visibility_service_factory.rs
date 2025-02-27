use crate::domain::services::security::toggle_comment_visibility_service::ToggleCommentVisibilityService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> ToggleCommentVisibilityService<SeaCommentRepository> {
    let comment_repository = SeaCommentRepository::new(db_conn);
    ToggleCommentVisibilityService::new(comment_repository)
}
