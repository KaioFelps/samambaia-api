use crate::domain::services::fetch_many_team_users_service::FetchManyTeamUsersService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FetchManyTeamUsersService<SeaTeamUserRepository> {
    let team_user_repository = SeaTeamUserRepository::new(db_conn);
    FetchManyTeamUsersService::new(team_user_repository)
}
