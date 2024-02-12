use crate::domain::services::delete_article_service::DeleteArticleService;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec<'sea_service_lf>() -> DeleteArticleService<SeaArticleRepository, SeaArticleCommentRepository, SeaUserRepository> {
    let sea_service = SeaService::new().await;
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);
    
    let sea_service = SeaService::new().await;
    let article_comment_repository = Box::new(SeaArticleCommentRepository::new(sea_service));

    let sea_service = SeaService::new().await;
    let article_repository = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let delete_article_service = DeleteArticleService::new(
        article_repository,
        article_comment_repository,
        user_repository
    );

    delete_article_service
}