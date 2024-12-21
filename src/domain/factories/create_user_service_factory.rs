use crate::domain::services::identity::create_user_service::CreateUserService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> CreateUserService<SeaUserRepository, PasswordAuthHasherAndVerifier> {
    let user_repository = SeaUserRepository::new(db_conn);
    let hasher = PasswordAuthHasherAndVerifier;

    CreateUserService::new(user_repository, hasher)
}
