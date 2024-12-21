use crate::domain::services::update_user_service::UpdateUserService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> UpdateUserService<SeaUserRepository, PasswordAuthHasherAndVerifier> {
    let user_repository = SeaUserRepository::new(db_conn);
    let hasher = PasswordAuthHasherAndVerifier;

    UpdateUserService::new(user_repository, hasher)
}
