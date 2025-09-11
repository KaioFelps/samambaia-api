use sea_orm::DatabaseConnection;

use crate::domain::services::journalism::article_tags::update_article_tag_service::UpdateArticleTagService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> UpdateArticleTagService<SeaArticleTagRepository<'_, DatabaseConnection>> {
    let article_tag_repository = SeaArticleTagRepository::new(&db_conn.db);
    UpdateArticleTagService::new(article_tag_repository)
}
