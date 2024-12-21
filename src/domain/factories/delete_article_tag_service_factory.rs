use crate::domain::services::journalism::article_tags::delete_article_tag_service::DeleteArticleTagService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> DeleteArticleTagService<SeaArticleTagRepository> {
    let article_tag_repository = SeaArticleTagRepository::new(db_conn);
    DeleteArticleTagService::new(article_tag_repository)
}
