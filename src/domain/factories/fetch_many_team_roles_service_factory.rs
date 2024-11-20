use crate::domain::services::fetch_many_team_roles_service::FetchManyTeamRolesService;
use crate::error::DomainError;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Result<FetchManyTeamRolesService<SeaTeamRoleRepository>, DomainError> {
    let sea_service = SeaService::new()
        .await
        .map_err(|_| DomainError::internal_err())?;

    let team_role_repository: Box<SeaTeamRoleRepository> =
        Box::new(SeaTeamRoleRepository::new(sea_service).await);

    let fetch_many_team_roles_service = FetchManyTeamRolesService::new(team_role_repository);

    Ok(fetch_many_team_roles_service)
}
