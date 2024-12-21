use crate::domain::services::identity::change_password_service::ChangePasswordService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(
    db_conn: &SeaService,
) -> ChangePasswordService<SeaUserRepository, PasswordAuthHasherAndVerifier> {
    let user_repository = SeaUserRepository::new(db_conn);
    let hasher_and_comparer = PasswordAuthHasherAndVerifier;

    ChangePasswordService::new(user_repository, hasher_and_comparer)
}
