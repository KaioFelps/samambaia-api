use crate::domain::services::fetch_many_articles_service::FetchManyArticlesService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
) -> Result<FetchManyArticlesService<SeaArticleRepository, SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let article_repository: Box<SeaArticleRepository> =
        Box::new(SeaArticleRepository::new(sea_service).await);

    let fetch_many_articles_service =
        FetchManyArticlesService::new(article_repository, user_repository);

    Ok(fetch_many_articles_service)
}
