use crate::domain::services::change_password_service::ChangePasswordService;
use crate::error::DomainError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<ChangePasswordService<SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let hasher_and_comparer: Box<PasswordAuthHasherAndVerifier> =
        Box::new(PasswordAuthHasherAndVerifier {});

    let change_password_service = ChangePasswordService::new(user_repository, hasher_and_comparer);

    Ok(change_password_service)
}
