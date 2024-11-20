use crate::domain::services::fetch_many_article_tags_service::FetchManyArticleTagsService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<FetchManyArticleTagsService<SeaArticleTagRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;
    let article_tag_repository = SeaArticleTagRepository::new(sea_service).await;

    let fetch_many_article_tags_service = FetchManyArticleTagsService::new(article_tag_repository);

    Ok(fetch_many_article_tags_service)
}
