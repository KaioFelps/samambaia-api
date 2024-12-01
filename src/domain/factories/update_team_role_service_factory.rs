use crate::domain::services::update_team_role_service::UpdateTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> UpdateTeamRoleService<SeaTeamRoleRepository> {
    let team_role_repository = Box::new(SeaTeamRoleRepository::new(db_conn).await);
    UpdateTeamRoleService::new(team_role_repository)
}
