use crate::domain::services::fetch_many_team_roles_service::FetchManyTeamRolesService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FetchManyTeamRolesService<SeaTeamRoleRepository> {
    let team_role_repository = Box::new(SeaTeamRoleRepository::new(db_conn));
    FetchManyTeamRolesService::new(team_role_repository)
}
