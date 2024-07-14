use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::domain::services::fetch_many_comments_with_author_service::FetchManyArticleCommentsWithAuthorService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchManyArticleCommentsWithAuthorService<SeaCommentUserArticleRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let comment_user_article_repository: Box<SeaCommentUserArticleRepository> = Box::new(SeaCommentUserArticleRepository::new(sea_service).await);

    let fetch_many_article_comments_service = FetchManyArticleCommentsWithAuthorService::new(
        comment_user_article_repository,
    );

    Left(fetch_many_article_comments_service)
}