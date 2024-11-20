use crate::domain::services::fetch_many_team_users_service::FetchManyTeamUsersService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<FetchManyTeamUsersService<SeaTeamUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let team_user_repository: Box<SeaTeamUserRepository> =
        Box::new(SeaTeamUserRepository::new(sea_service).await);

    let fetch_many_team_users_service = FetchManyTeamUsersService::new(team_user_repository);

    Ok(fetch_many_team_users_service)
}
