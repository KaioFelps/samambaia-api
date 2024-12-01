use crate::domain::services::create_user_service::CreateUserService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> CreateUserService<SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    let hasher = Box::new(PasswordAuthHasherAndVerifier {});

    CreateUserService::new(user_repository, hasher)
}
