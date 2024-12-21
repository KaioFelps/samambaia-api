use crate::domain::services::journalism::comments::comment_on_article_service::CommentOnArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> CommentOnArticleService<SeaCommentRepository, SeaArticleRepository> {
    let comment_repository = SeaCommentRepository::new(db_conn);
    let article_repository = SeaArticleRepository::new(db_conn);

    CommentOnArticleService::new(comment_repository, article_repository)
}
