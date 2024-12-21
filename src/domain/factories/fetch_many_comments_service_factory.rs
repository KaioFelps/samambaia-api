use crate::domain::services::fetch_many_comments_service::FetchManyCommentsService;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> FetchManyCommentsService<SeaArticleCommentRepository, SeaUserRepository> {
    let user_repository = SeaUserRepository::new(db_conn);
    let article_comment_repository = SeaArticleCommentRepository::new(db_conn);

    FetchManyCommentsService::new(article_comment_repository, user_repository)
}
