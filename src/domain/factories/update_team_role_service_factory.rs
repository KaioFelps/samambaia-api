use crate::domain::services::update_team_role_service::UpdateTeamRoleService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<UpdateTeamRoleService<SeaTeamRoleRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let team_role_repository = Box::new(SeaTeamRoleRepository::new(sea_service).await);

    let update_team_role_service = UpdateTeamRoleService::new(team_role_repository);

    Ok(update_team_role_service)
}
