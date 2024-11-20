use crate::domain::services::comment_on_article_service::CommentOnArticleService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
) -> Result<CommentOnArticleService<SeaCommentRepository, SeaArticleRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let comment_repository: Box<SeaCommentRepository> =
        Box::new(SeaCommentRepository::new(sea_service.clone()).await);
    let article_repository: Box<SeaArticleRepository> =
        Box::new(SeaArticleRepository::new(sea_service).await);

    let comment_on_article_service =
        CommentOnArticleService::new(comment_repository, article_repository);

    Ok(comment_on_article_service)
}
