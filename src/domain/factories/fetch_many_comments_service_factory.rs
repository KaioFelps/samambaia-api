use crate::domain::services::fetch_many_comments_service::FetchManyCommentsService;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> FetchManyCommentsService<SeaArticleCommentRepository, SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    let article_comment_repository = Box::new(SeaArticleCommentRepository::new(db_conn).await);

    FetchManyCommentsService::new(article_comment_repository, user_repository)
}
