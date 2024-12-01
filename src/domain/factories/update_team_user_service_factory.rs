use crate::domain::services::update_team_user_service::UpdateTeamUserService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> UpdateTeamUserService<SeaTeamUserRepository> {
    let team_user_repository = Box::new(SeaTeamUserRepository::new(db_conn).await);
    UpdateTeamUserService::new(team_user_repository)
}
