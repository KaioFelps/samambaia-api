use crate::domain::services::fetch_many_articles_service::FetchManyArticlesService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> FetchManyArticlesService<SeaArticleRepository, SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    let article_repository = Box::new(SeaArticleRepository::new(db_conn).await);

    FetchManyArticlesService::new(article_repository, user_repository)
}
