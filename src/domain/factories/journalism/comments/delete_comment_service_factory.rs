use crate::domain::services::journalism::comments::delete_comment_service::DeleteCommentService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> DeleteCommentService<SeaCommentRepository> {
    let comment_repository = SeaCommentRepository::new(db_conn);
    DeleteCommentService::new(comment_repository)
}
