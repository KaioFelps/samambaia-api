use crate::domain::services::fetch_many_comments_service::FetchManyCommentsService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> Either<FetchManyCommentsService<SeaArticleCommentRepository, SeaUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let article_comment_repository: Box<SeaArticleCommentRepository> = Box::new(SeaArticleCommentRepository::new(sea_service).await);
    
    let fetch_many_comments_service = FetchManyCommentsService::new(
        article_comment_repository,
        user_repository
    );

    Left(fetch_many_comments_service)
}