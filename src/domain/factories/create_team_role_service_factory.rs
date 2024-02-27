use crate::domain::services::create_team_role_service::CreateTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> CreateTeamRoleService<SeaTeamRoleRepository> {
    let sea_service = SeaService::new().await;

    let sea_team_role_repository: Box<SeaTeamRoleRepository> = Box::new(SeaTeamRoleRepository::new(sea_service).await);

    let create_team_role_service = CreateTeamRoleService::new(
        sea_team_role_repository,
    );

    create_team_role_service
}