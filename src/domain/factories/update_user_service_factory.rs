use crate::domain::services::update_user_service::UpdateUserService;
use crate::error::DomainError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<UpdateUserService<SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service).await);

    let hasher = Box::new(PasswordAuthHasherAndVerifier {});

    let update_user_service = UpdateUserService::new(user_repository, hasher);

    Ok(update_user_service)
}
