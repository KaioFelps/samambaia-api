use crate::domain::services::get_user_service::GetUserService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> Either<GetUserService<SeaUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    
    let user_repository: Box<SeaUserRepository> =
    Box::new(SeaUserRepository::new(sea_service.clone()).await);
    
    let get_user_service = GetUserService::new(
        user_repository,
    );

    Left(get_user_service)
}