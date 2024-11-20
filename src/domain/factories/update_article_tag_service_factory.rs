use crate::domain::services::update_article_tag_service::UpdateArticleTagService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<UpdateArticleTagService<SeaArticleTagRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let article_tag_repository = SeaArticleTagRepository::new(sea_service).await;

    let update_article_tag_service = UpdateArticleTagService::new(article_tag_repository);

    Ok(update_article_tag_service)
}
