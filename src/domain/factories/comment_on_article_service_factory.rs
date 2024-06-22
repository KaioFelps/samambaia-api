use crate::domain::services::comment_on_article_service::CommentOnArticleService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_repository::SeaCommentRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<CommentOnArticleService<SeaCommentRepository, SeaArticleRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let comment_repository: Box<SeaCommentRepository> = Box::new(SeaCommentRepository::new(sea_service.clone()).await);
    let article_repository: Box<SeaArticleRepository> = Box::new(SeaArticleRepository::new(sea_service).await);
    
    let comment_on_article_service = CommentOnArticleService::new(comment_repository, article_repository);

    Left(comment_on_article_service)
}