use crate::infra::jwt::jwt_service::JwtService;
use crate::services::authenticate_user_service::AuthenticateUserService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> AuthenticateUserService<SeaUserRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);
    
    let jwt_service = JwtService {};

    let authenticate_user_service = AuthenticateUserService::new(user_repository, Box::new(jwt_service));

    authenticate_user_service
}