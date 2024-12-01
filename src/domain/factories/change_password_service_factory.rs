use crate::domain::services::change_password_service::ChangePasswordService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> ChangePasswordService<SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    let hasher_and_comparer = Box::new(PasswordAuthHasherAndVerifier {});

    ChangePasswordService::new(user_repository, hasher_and_comparer)
}
