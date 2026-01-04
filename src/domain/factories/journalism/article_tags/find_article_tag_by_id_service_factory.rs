use sea_orm::DatabaseConnection;

use crate::domain::services::journalism::article_tags::find_article_tag_by_id_service::FindArticleTagByIdService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> FindArticleTagByIdService<SeaArticleTagRepository<'_, DatabaseConnection>> {
    let article_tag_repository = SeaArticleTagRepository::new(&db_conn.db);
    FindArticleTagByIdService::new(article_tag_repository)
}
