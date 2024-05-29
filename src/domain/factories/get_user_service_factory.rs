use crate::domain::services::get_user_service::GetUserService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> GetUserService<SeaUserRepository> {
    let sea_service = SeaService::new().await;
    
    let user_repository: Box<SeaUserRepository> =
    Box::new(SeaUserRepository::new(sea_service.clone()).await);
    
    let get_user_service = GetUserService::new(
        user_repository,
    );

    get_user_service
}