use crate::domain::services::journalism::articles::update_article_service::UpdateArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> UpdateArticleService<SeaArticleRepository, SeaArticleTagRepository> {
    let article_repository = SeaArticleRepository::new(db_conn);
    let article_tag_repository = SeaArticleTagRepository::new(db_conn);

    UpdateArticleService::new(article_repository, article_tag_repository)
}
