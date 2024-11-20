use crate::domain::services::delete_article_tag_service::DeleteArticleTagService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<DeleteArticleTagService<SeaArticleTagRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let article_tag_repository = SeaArticleTagRepository::new(sea_service).await;

    let delete_article_tag_service = DeleteArticleTagService::new(article_tag_repository);

    Ok(delete_article_tag_service)
}
