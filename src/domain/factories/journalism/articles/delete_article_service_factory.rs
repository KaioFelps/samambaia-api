use crate::domain::services::journalism::articles::delete_article_service::DeleteArticleService;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> DeleteArticleService<SeaArticleRepository, SeaArticleCommentRepository> {
    let article_comment_repository = SeaArticleCommentRepository::new(db_conn);
    let article_repository = SeaArticleRepository::new(db_conn);

    DeleteArticleService::new(article_repository, article_comment_repository)
}
