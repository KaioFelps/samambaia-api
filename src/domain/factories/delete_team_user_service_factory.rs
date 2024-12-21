use crate::domain::services::delete_team_user_service::DeleteTeamUserService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> DeleteTeamUserService<SeaTeamUserRepository> {
    let team_user_repository = Box::new(SeaTeamUserRepository::new(db_conn));
    DeleteTeamUserService::new(team_user_repository)
}
