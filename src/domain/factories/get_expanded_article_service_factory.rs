use crate::domain::services::get_expanded_article_service::GetExpandedArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> GetExpandedArticleService<
    SeaUserRepository,
    SeaArticleRepository,
    SeaCommentUserArticleRepository,
> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    let article_repository = Box::new(SeaArticleRepository::new(db_conn).await);
    let comment_user_article_repository =
        Box::new(SeaCommentUserArticleRepository::new(db_conn).await);

    GetExpandedArticleService::new(
        user_repository,
        article_repository,
        comment_user_article_repository,
    )
}
