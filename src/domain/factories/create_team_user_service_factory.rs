use crate::domain::services::create_team_user_service::CreateTeamUserService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(
    db_conn: &SeaService,
) -> CreateTeamUserService<SeaTeamUserRepository, SeaTeamRoleRepository> {
    let team_role_repository = Box::new(SeaTeamRoleRepository::new(db_conn).await);
    let team_user_repository = Box::new(SeaTeamUserRepository::new(db_conn).await);

    CreateTeamUserService::new(team_user_repository, team_role_repository)
}
