use sea_orm::DatabaseConnection;

use crate::domain::services::journalism::articles::create_article_service::CreateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> CreateArticleService<SeaArticleRepository, SeaArticleTagRepository<DatabaseConnection>> {
    let sea_article_repository = SeaArticleRepository::new(db_conn);
    let sea_article_tag_repository = SeaArticleTagRepository::new(&db_conn.db);

    CreateArticleService::new(sea_article_repository, sea_article_tag_repository)
}
