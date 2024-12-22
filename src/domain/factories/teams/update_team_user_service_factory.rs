use crate::domain::services::teams::update_team_user_service::UpdateTeamUserService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> UpdateTeamUserService<SeaTeamUserRepository> {
    let team_user_repository = SeaTeamUserRepository::new(db_conn);
    UpdateTeamUserService::new(team_user_repository)
}
