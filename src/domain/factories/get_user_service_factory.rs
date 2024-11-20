use crate::domain::services::get_user_service::GetUserService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<GetUserService<SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service.clone()).await);

    let get_user_service = GetUserService::new(user_repository);

    Ok(get_user_service)
}
