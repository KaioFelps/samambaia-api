use crate::domain::services::update_team_user_service::UpdateTeamUserService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> UpdateTeamUserService<SeaTeamUserRepository> {
    let sea_service = SeaService::new().await;
    
    let team_user_repository = Box::new(SeaTeamUserRepository::new(sea_service).await);
    
    let update_team_user_service = UpdateTeamUserService::new(
        team_user_repository,
    );

    update_team_user_service
}