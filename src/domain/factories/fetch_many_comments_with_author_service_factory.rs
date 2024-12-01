use crate::domain::services::fetch_many_comments_with_author_service::FetchManyArticleCommentsWithAuthorService;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> FetchManyArticleCommentsWithAuthorService<SeaCommentUserArticleRepository> {
    let comment_user_article_repository =
        Box::new(SeaCommentUserArticleRepository::new(db_conn).await);

    FetchManyArticleCommentsWithAuthorService::new(comment_user_article_repository)
}
