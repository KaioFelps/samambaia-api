use crate::domain::services::update_article_service::UpdateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> UpdateArticleService<SeaArticleRepository, SeaUserRepository> {
    let sea_service = SeaService::new().await;
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);
        
    let sea_service = SeaService::new().await;
    let article_repository = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let update_article_service = UpdateArticleService::new(article_repository, user_repository);

    update_article_service
}