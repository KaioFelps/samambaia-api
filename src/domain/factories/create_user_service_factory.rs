use crate::domain::services::create_user_service::CreateUserService;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> CreateUserService<SeaUserRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);

    let hasher = Box::new(PasswordAuthHasherAndVerifier {});
    
    let create_user_service = CreateUserService::new(user_repository, hasher);

    create_user_service
}