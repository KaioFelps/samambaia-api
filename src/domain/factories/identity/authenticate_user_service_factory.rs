use crate::domain::services::identity::authenticate_user_service::AuthenticateUserService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::jwt::jwt_service::JwtService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> AuthenticateUserService<SeaUserRepository, PasswordAuthHasherAndVerifier> {
    let user_repository = SeaUserRepository::new(db_conn);

    let jwt_service = JwtService;

    let verifier = PasswordAuthHasherAndVerifier;

    AuthenticateUserService::new(user_repository, jwt_service, verifier)
}
