use crate::domain::services::update_article_service::UpdateArticleService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
) -> Result<UpdateArticleService<SeaArticleRepository, SeaArticleTagRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let article_repository = Box::new(SeaArticleRepository::new(sea_service.clone()).await);
    let article_tag_repository = Box::new(SeaArticleTagRepository::new(sea_service).await);

    let update_article_service =
        UpdateArticleService::new(article_repository, article_tag_repository);

    Ok(update_article_service)
}
