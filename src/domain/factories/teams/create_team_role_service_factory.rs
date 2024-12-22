use crate::domain::services::teams::create_team_role_service::CreateTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> CreateTeamRoleService<SeaTeamRoleRepository> {
    let sea_team_role_repository = SeaTeamRoleRepository::new(db_conn);
    CreateTeamRoleService::new(sea_team_role_repository)
}
