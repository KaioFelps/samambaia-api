use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::services::update_user_service::UpdateUserService;

pub async fn exec() -> UpdateUserService {
    let sea_service = SeaService::new().await;

    let user_repository: SeaUserRepository = SeaUserRepository::new(sea_service).await;
    
    let update_user_service = UpdateUserService::new(user_repository);

    update_user_service
}