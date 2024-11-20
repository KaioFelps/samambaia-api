use crate::domain::services::authenticate_user_service::AuthenticateUserService;
use crate::error::DomainError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::jwt::jwt_service::JwtService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<AuthenticateUserService<SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let jwt_service = JwtService {};

    let verifier = Box::new(PasswordAuthHasherAndVerifier {});

    let authenticate_user_service =
        AuthenticateUserService::new(user_repository, Box::new(jwt_service), verifier);

    Ok(authenticate_user_service)
}
