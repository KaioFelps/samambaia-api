use crate::domain::services::fetch_many_team_roles_service::FetchManyTeamRolesService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchManyTeamRolesService<SeaTeamRoleRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let team_role_repository: Box<SeaTeamRoleRepository> = Box::new(SeaTeamRoleRepository::new(sea_service).await);
    
    let fetch_many_team_roles_service = FetchManyTeamRolesService::new(team_role_repository);

    Left(fetch_many_team_roles_service)
}
