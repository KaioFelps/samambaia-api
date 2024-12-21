use crate::domain::services::journalism::articles::fetch_many_articles_service::FetchManyArticlesService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> FetchManyArticlesService<SeaArticleRepository, SeaUserRepository> {
    let user_repository = SeaUserRepository::new(db_conn);
    let article_repository = SeaArticleRepository::new(db_conn);

    FetchManyArticlesService::new(article_repository, user_repository)
}
