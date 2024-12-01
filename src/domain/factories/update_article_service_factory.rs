use crate::domain::services::update_article_service::UpdateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> UpdateArticleService<SeaArticleRepository, SeaArticleTagRepository> {
    let article_repository = Box::new(SeaArticleRepository::new(db_conn).await);
    let article_tag_repository = Box::new(SeaArticleTagRepository::new(db_conn).await);

    UpdateArticleService::new(article_repository, article_tag_repository)
}
