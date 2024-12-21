use crate::domain::services::teams::create_team_user_service::CreateTeamUserService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> CreateTeamUserService<SeaTeamUserRepository, SeaTeamRoleRepository> {
    let team_role_repository = SeaTeamRoleRepository::new(db_conn);
    let team_user_repository = SeaTeamUserRepository::new(db_conn);

    CreateTeamUserService::new(team_user_repository, team_role_repository)
}
