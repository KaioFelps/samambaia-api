use crate::domain::services::journalism::article_tags::create_article_tag_service::CreateArticleTagService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> CreateArticleTagService<SeaArticleTagRepository> {
    let sea_article_tag_repository = SeaArticleTagRepository::new(db_conn);
    CreateArticleTagService::new(sea_article_tag_repository)
}
