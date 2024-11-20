use crate::domain::services::create_article_service::CreateArticleService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec<'a>() -> Result<
    CreateArticleService<SeaArticleRepository, SeaArticleTagRepository, SeaUserRepository>,
    DomainError,
> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let sea_article_repository: Box<SeaArticleRepository> =
        Box::new(SeaArticleRepository::new(sea_service.clone()).await);
    let sea_article_tag_repository: Box<SeaArticleTagRepository> =
        Box::new(SeaArticleTagRepository::new(sea_service.clone()).await);
    let sea_user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let create_article_service: CreateArticleService<
        SeaArticleRepository,
        SeaArticleTagRepository,
        SeaUserRepository,
    > = CreateArticleService::new(
        sea_article_repository,
        sea_article_tag_repository,
        sea_user_repository,
    );

    Ok(create_article_service)
}
