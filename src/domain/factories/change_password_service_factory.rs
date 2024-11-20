use crate::domain::services::change_password_service::ChangePasswordService;
use crate::errors::internal_error::InternalError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec() -> Either<ChangePasswordService<SeaUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let hasher_and_comparer: Box<PasswordAuthHasherAndVerifier> =
        Box::new(PasswordAuthHasherAndVerifier {});

    let change_password_service = ChangePasswordService::new(user_repository, hasher_and_comparer);

    Left(change_password_service)
}
