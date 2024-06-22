use actix_web::HttpResponse;
use either::Either::{self, Left, Right};
use crate::errors::internal_error::InternalError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::jwt::jwt_service::JwtService;
use crate::domain::services::authenticate_user_service::AuthenticateUserService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> Either<AuthenticateUserService<SeaUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);
    
    let jwt_service = JwtService {};

    let verifier = Box::new(PasswordAuthHasherAndVerifier {});

    let authenticate_user_service = AuthenticateUserService::new(user_repository, Box::new(jwt_service), verifier);

    Left(authenticate_user_service)
}