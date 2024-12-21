use crate::domain::services::teams::update_team_role_service::UpdateTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> UpdateTeamRoleService<SeaTeamRoleRepository> {
    let team_role_repository = SeaTeamRoleRepository::new(db_conn);
    UpdateTeamRoleService::new(team_role_repository)
}
