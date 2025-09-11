use crate::domain::services::journalism::articles::find_article_by_id_service::FindArticleByIdService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FindArticleByIdService<SeaArticleRepository<'_>> {
    let article_repository = SeaArticleRepository::new(db_conn);

    FindArticleByIdService::new(article_repository)
}
