use crate::domain::services::fetch_many_users_service::FetchManyUsersService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<FetchManyUsersService<SeaUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let user_repository: Box<SeaUserRepository> =
        Box::new(SeaUserRepository::new(sea_service.clone()).await);

    let fetch_many_users_service = FetchManyUsersService::new(user_repository);

    Ok(fetch_many_users_service)
}
