use crate::domain::services::fetch_many_comments_service::FetchManyCommentsService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
) -> Result<FetchManyCommentsService<SeaArticleCommentRepository, SeaUserRepository>, DomainError>
{
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let article_comment_repository: Box<SeaArticleCommentRepository> =
        Box::new(SeaArticleCommentRepository::new(sea_service).await);

    let fetch_many_comments_service =
        FetchManyCommentsService::new(article_comment_repository, user_repository);

    Ok(fetch_many_comments_service)
}
