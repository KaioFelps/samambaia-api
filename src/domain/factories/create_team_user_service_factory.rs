use crate::domain::services::create_team_user_service::CreateTeamUserService;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;
use actix_web::HttpResponse;
use either::Either::{self, *};

pub async fn exec(
) -> Either<CreateTeamUserService<SeaTeamUserRepository, SeaTeamRoleRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(
            InternalError::new(),
        )));
    }

    let sea_service = sea_service.unwrap();

    let team_role_repository = Box::new(SeaTeamRoleRepository::new(sea_service.clone()).await);
    let team_user_repository = Box::new(SeaTeamUserRepository::new(sea_service).await);

    let create_team_user_service =
        CreateTeamUserService::new(team_user_repository, team_role_repository);

    Left(create_team_user_service)
}
