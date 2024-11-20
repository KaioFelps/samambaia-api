use crate::domain::services::delete_team_user_service::DeleteTeamUserService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<DeleteTeamUserService<SeaTeamUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let team_user_repository: Box<SeaTeamUserRepository> =
        Box::new(SeaTeamUserRepository::new(sea_service).await);

    let delete_team_user_service = DeleteTeamUserService::new(team_user_repository);

    Ok(delete_team_user_service)
}
