use crate::domain::services::delete_team_role_service::DeleteTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> DeleteTeamRoleService<SeaTeamRoleRepository> {
    let team_role_repository = Box::new(SeaTeamRoleRepository::new(db_conn).await);
    DeleteTeamRoleService::new(team_role_repository)
}
