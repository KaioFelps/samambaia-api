use crate::domain::services::update_team_user_service::UpdateTeamUserService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<UpdateTeamUserService<SeaTeamUserRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let team_user_repository = Box::new(SeaTeamUserRepository::new(sea_service).await);

    let update_team_user_service = UpdateTeamUserService::new(team_user_repository);

    Ok(update_team_user_service)
}
