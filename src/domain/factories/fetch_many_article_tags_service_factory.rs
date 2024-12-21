use crate::domain::services::fetch_many_article_tags_service::FetchManyArticleTagsService;
use crate::infra::sea::repositories::sea_article_tag_repository::SeaArticleTagRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FetchManyArticleTagsService<SeaArticleTagRepository> {
    let article_tag_repository = SeaArticleTagRepository::new(db_conn);
    FetchManyArticleTagsService::new(article_tag_repository)
}
