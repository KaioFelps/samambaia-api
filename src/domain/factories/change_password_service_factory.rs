use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::domain::services::change_password_service::ChangePasswordService;

pub async fn exec() -> ChangePasswordService<SeaUserRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);

    let hasher_and_comparer: Box<PasswordAuthHasherAndVerifier> = Box::new(PasswordAuthHasherAndVerifier {});
    
    let change_password_service = ChangePasswordService::new(user_repository, hasher_and_comparer);

    change_password_service
}