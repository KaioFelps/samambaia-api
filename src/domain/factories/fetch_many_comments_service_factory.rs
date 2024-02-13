use crate::domain::services::fetch_many_comments_service::FetchManyCommentsService;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> FetchManyCommentsService<SeaArticleCommentRepository, SeaUserRepository> {
    let sea_service = SeaService::new().await;
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);
        
    let sea_service = SeaService::new().await;
    let article_comment_repository = Box::new(SeaArticleCommentRepository::new(sea_service).await);
    
    let fetch_many_articles_service = FetchManyCommentsService::new(article_comment_repository, user_repository);

    fetch_many_articles_service
}