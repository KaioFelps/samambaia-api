use crate::domain::services::fetch_many_team_users_service::FetchManyTeamUsersService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<FetchManyTeamUsersService<SeaTeamUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let team_user_repository: Box<SeaTeamUserRepository> = Box::new(SeaTeamUserRepository::new(sea_service).await);
    
    let fetch_many_team_users_service = FetchManyTeamUsersService::new(team_user_repository);

    Left(fetch_many_team_users_service)
}
