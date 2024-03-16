use crate::domain::services::create_team_user_service::CreateTeamUserService;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> CreateTeamUserService<SeaTeamUserRepository, SeaTeamRoleRepository> {
    let sea_service = SeaService::new().await;
    
    let team_role_repository = Box::new(SeaTeamRoleRepository::new(sea_service.clone()).await);
    let team_user_repository = Box::new(SeaTeamUserRepository::new(sea_service).await);
    
    let create_team_user_service = CreateTeamUserService::new(
        team_user_repository,
        team_role_repository,
    );

    create_team_user_service
}