use crate::domain::services::create_article_service::CreateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> CreateArticleService<SeaArticleRepository, SeaArticleTagRepository, SeaUserRepository> {
    let sea_article_repository = Box::new(SeaArticleRepository::new(db_conn).await);
    let sea_article_tag_repository = Box::new(SeaArticleTagRepository::new(db_conn).await);
    let sea_user_repository = Box::new(SeaUserRepository::new(db_conn).await);

    CreateArticleService::new(
        sea_article_repository,
        sea_article_tag_repository,
        sea_user_repository,
    )
}
