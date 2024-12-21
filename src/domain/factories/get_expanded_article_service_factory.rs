use crate::domain::services::get_expanded_article_service::GetExpandedArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> GetExpandedArticleService<
    SeaUserRepository,
    SeaArticleRepository,
    SeaCommentUserArticleRepository,
> {
    let user_repository = SeaUserRepository::new(db_conn);
    let article_repository = SeaArticleRepository::new(db_conn);
    let comment_user_article_repository = SeaCommentUserArticleRepository::new(db_conn);

    GetExpandedArticleService::new(
        user_repository,
        article_repository,
        comment_user_article_repository,
    )
}
