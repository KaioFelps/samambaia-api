use crate::domain::services::delete_article_tag_service::DeleteArticleTagService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> DeleteArticleTagService<SeaArticleTagRepository> {
    let article_tag_repository = SeaArticleTagRepository::new(db_conn).await;
    DeleteArticleTagService::new(article_tag_repository)
}
