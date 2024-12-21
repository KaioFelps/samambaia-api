use crate::domain::services::update_article_tag_service::UpdateArticleTagService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> UpdateArticleTagService<SeaArticleTagRepository> {
    let article_tag_repository = SeaArticleTagRepository::new(db_conn);
    UpdateArticleTagService::new(article_tag_repository)
}
