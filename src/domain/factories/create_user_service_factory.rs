use crate::domain::services::create_user_service::CreateUserService;
use crate::error::DomainError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<CreateUserService<SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let hasher = Box::new(PasswordAuthHasherAndVerifier {});

    let create_user_service = CreateUserService::new(user_repository, hasher);

    Ok(create_user_service)
}
