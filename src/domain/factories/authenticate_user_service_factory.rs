use crate::domain::services::authenticate_user_service::AuthenticateUserService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::jwt::jwt_service::JwtService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> AuthenticateUserService<SeaUserRepository> {
    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(db_conn).await);

    let jwt_service = JwtService {};

    let verifier = Box::new(PasswordAuthHasherAndVerifier {});

    AuthenticateUserService::new(user_repository, Box::new(jwt_service), verifier)
}
