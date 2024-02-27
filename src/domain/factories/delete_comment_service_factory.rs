use crate::domain::services::delete_comment_service::DeleteCommentService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> DeleteCommentService<SeaCommentRepository> {
    let sea_service = SeaService::new().await;

    let comment_repository: Box<SeaCommentRepository> = Box::new(SeaCommentRepository::new(sea_service).await);
    
    let delete_comment_service = DeleteCommentService::new(comment_repository);

    delete_comment_service
}