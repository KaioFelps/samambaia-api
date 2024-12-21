use crate::domain::services::toggle_comment_visibility_service::ToggleCommentVisibilityService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> ToggleCommentVisibilityService<SeaCommentRepository> {
    let comment_repository = Box::new(SeaCommentRepository::new(db_conn));
    ToggleCommentVisibilityService::new(comment_repository)
}
