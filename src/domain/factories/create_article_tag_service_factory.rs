use crate::domain::services::create_article_tag_service::CreateArticleTagService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> CreateArticleTagService<SeaArticleTagRepository> {
    let sea_article_tag_repository = SeaArticleTagRepository::new(db_conn).await;
    CreateArticleTagService::new(sea_article_tag_repository)
}
