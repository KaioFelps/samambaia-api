use crate::domain::services::fetch_many_comments_with_author_service::FetchManyArticleCommentsWithAuthorService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
) -> Result<FetchManyArticleCommentsWithAuthorService<SeaCommentUserArticleRepository>, DomainError>
{
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let comment_user_article_repository: Box<SeaCommentUserArticleRepository> =
        Box::new(SeaCommentUserArticleRepository::new(sea_service).await);

    let fetch_many_article_comments_service =
        FetchManyArticleCommentsWithAuthorService::new(comment_user_article_repository);

    Ok(fetch_many_article_comments_service)
}
