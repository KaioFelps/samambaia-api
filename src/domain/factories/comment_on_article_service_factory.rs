use crate::domain::services::comment_on_article_service::CommentOnArticleService;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> CommentOnArticleService<SeaCommentRepository, SeaArticleRepository> {
    let comment_repository = Box::new(SeaCommentRepository::new(db_conn).await);
    let article_repository = Box::new(SeaArticleRepository::new(db_conn).await);

    CommentOnArticleService::new(comment_repository, article_repository)
}
