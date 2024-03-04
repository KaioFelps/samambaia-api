use crate::domain::services::update_team_role_service::UpdateTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> UpdateTeamRoleService<SeaTeamRoleRepository> {
    let sea_service = SeaService::new().await;
    
    let team_role_repository = Box::new(SeaTeamRoleRepository::new(sea_service).await);
    
    let update_team_role_service = UpdateTeamRoleService::new(
        team_role_repository,
    );

    update_team_role_service
}