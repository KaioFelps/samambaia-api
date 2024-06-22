use crate::domain::services::update_team_user_service::UpdateTeamUserService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::sea::repositories::sea_team_user_repository::SeaTeamUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> Either<UpdateTeamUserService<SeaTeamUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();
    
    let team_user_repository = Box::new(SeaTeamUserRepository::new(sea_service).await);
    
    let update_team_user_service = UpdateTeamUserService::new(
        team_user_repository,
    );

    Left(update_team_user_service)
}