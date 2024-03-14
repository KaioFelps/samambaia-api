use crate::domain::services::fetch_many_team_roles_service::FetchManyTeamRolesService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> FetchManyTeamRolesService<SeaTeamRoleRepository> {
    let sea_service = SeaService::new().await;

    let team_role_repository: Box<SeaTeamRoleRepository> = Box::new(SeaTeamRoleRepository::new(sea_service).await);
    
    let fetch_many_team_roles_service = FetchManyTeamRolesService::new(team_role_repository);

    fetch_many_team_roles_service
}
