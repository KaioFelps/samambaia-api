use crate::domain::services::create_team_role_service::CreateTeamRoleService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<CreateTeamRoleService<SeaTeamRoleRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let sea_team_role_repository: Box<SeaTeamRoleRepository> =
        Box::new(SeaTeamRoleRepository::new(sea_service).await);

    let create_team_role_service = CreateTeamRoleService::new(sea_team_role_repository);

    Ok(create_team_role_service)
}
