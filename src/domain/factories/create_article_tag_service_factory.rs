use crate::domain::services::create_article_tag_service::CreateArticleTagService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<CreateArticleTagService<SeaArticleTagRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let sea_article_tag_repository: SeaArticleTagRepository =
        SeaArticleTagRepository::new(sea_service).await;

    let create_article_tag_service = CreateArticleTagService::new(sea_article_tag_repository);

    Ok(create_article_tag_service)
}
