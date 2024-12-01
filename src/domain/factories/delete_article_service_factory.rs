use crate::domain::services::delete_article_service::DeleteArticleService;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> DeleteArticleService<SeaArticleRepository, SeaArticleCommentRepository, SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    let article_comment_repository = Box::new(SeaArticleCommentRepository::new(db_conn).await);
    let article_repository = Box::new(SeaArticleRepository::new(db_conn).await);

    DeleteArticleService::new(
        article_repository,
        article_comment_repository,
        user_repository,
    )
}
