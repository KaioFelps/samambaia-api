use crate::domain::services::get_expanded_article_service::GetExpandedArticleService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_comment_user_article_repository::SeaCommentUserArticleRepository;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> Either<GetExpandedArticleService<SeaUserRepository, SeaArticleRepository, SeaCommentUserArticleRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    
    let user_repository: Box<SeaUserRepository> =
    Box::new(SeaUserRepository::new(sea_service.clone()).await);

    let article_repository: Box<SeaArticleRepository> =
    Box::new(SeaArticleRepository::new(sea_service.clone()).await);
    
    let comment_user_article_repository: Box<SeaCommentUserArticleRepository> =
    Box::new(SeaCommentUserArticleRepository::new(sea_service).await);
    
    let get_expanded_article_service = GetExpandedArticleService::new(
        user_repository,
        article_repository,
        comment_user_article_repository
    );

    Left(get_expanded_article_service)
}