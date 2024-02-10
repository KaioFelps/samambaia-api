use crate::domain::services::delete_comment_service::DeleteCommentService;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> DeleteCommentService<SeaCommentRepository, SeaUserRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let comment_repository: Box<SeaCommentRepository> = Box::new(SeaCommentRepository::new(sea_service).await);
    
    let delete_comment_service = DeleteCommentService::new(comment_repository, user_repository);

    delete_comment_service
}