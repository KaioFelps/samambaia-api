use crate::domain::services::create_article_service::CreateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> CreateArticleService<SeaArticleRepository, SeaArticleTagRepository, SeaUserRepository> {
    let sea_article_repository = SeaArticleRepository::new(db_conn);
    let sea_article_tag_repository = SeaArticleTagRepository::new(db_conn);
    let sea_user_repository = SeaUserRepository::new(db_conn);

    CreateArticleService::new(
        sea_article_repository,
        sea_article_tag_repository,
        sea_user_repository,
    )
}
