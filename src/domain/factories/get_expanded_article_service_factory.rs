use crate::domain::services::get_expanded_article_service::GetExpandedArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> GetExpandedArticleService<SeaUserRepository, SeaArticleRepository, SeaCommentUserArticleRepository> {
    let sea_service = SeaService::new().await;
    
    let user_repository: Box<SeaUserRepository> =
    Box::new(SeaUserRepository::new(sea_service.clone()).await);

    let article_repository: Box<SeaArticleRepository> =
    Box::new(SeaArticleRepository::new(sea_service.clone()).await);
    
    let comment_user_article_repository: Box<SeaCommentUserArticleRepository> =
    Box::new(SeaCommentUserArticleRepository::new(sea_service).await);
    
    let get_expanded_article_service = GetExpandedArticleService::new(
        user_repository,
        article_repository,
        comment_user_article_repository
    );

    get_expanded_article_service
}