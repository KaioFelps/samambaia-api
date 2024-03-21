use crate::domain::services::fetch_many_team_users_service::FetchManyTeamUsersService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> FetchManyTeamUsersService<SeaTeamUserRepository> {
    let sea_service = SeaService::new().await;

    let team_user_repository: Box<SeaTeamUserRepository> = Box::new(SeaTeamUserRepository::new(sea_service).await);
    
    let fetch_many_team_users_service = FetchManyTeamUsersService::new(team_user_repository);

    fetch_many_team_users_service
}
