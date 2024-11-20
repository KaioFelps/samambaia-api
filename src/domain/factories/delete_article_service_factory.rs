use crate::domain::services::delete_article_service::DeleteArticleService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_article_comment_repository::SeaArticleCommentRepository;
use crate::infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<
    DeleteArticleService<SeaArticleRepository, SeaArticleCommentRepository, SeaUserRepository>,
    HttpResponse,
> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service.clone()).await);
    let article_comment_repository: Box<SeaArticleCommentRepository> =
        Box::new(SeaArticleCommentRepository::new(sea_service.clone()).await);
    let article_repository: Box<SeaArticleRepository> =
        Box::new(SeaArticleRepository::new(sea_service).await);

    let delete_article_service = DeleteArticleService::new(
        article_repository,
        article_comment_repository,
        user_repository,
    );

    Left(delete_article_service)
}
