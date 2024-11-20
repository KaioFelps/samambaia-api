use crate::domain::services::delete_article_service::DeleteArticleService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<
    DeleteArticleService<SeaArticleRepository, SeaArticleCommentRepository, SeaUserRepository>,
    DomainError,
> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let article_comment_repository: Box<SeaArticleCommentRepository> =
        Box::new(SeaArticleCommentRepository::new(sea_service.clone()).await);
    let article_repository: Box<SeaArticleRepository> =
        Box::new(SeaArticleRepository::new(sea_service).await);

    let delete_article_service = DeleteArticleService::new(
        article_repository,
        article_comment_repository,
        user_repository,
    );

    Ok(delete_article_service)
}
