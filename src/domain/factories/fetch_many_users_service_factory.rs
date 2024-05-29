use crate::domain::services::fetch_many_users_service::FetchManyUsersService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> FetchManyUsersService<SeaUserRepository> {
    let sea_service = SeaService::new().await;
    
    let user_repository: Box<SeaUserRepository> =
    Box::new(SeaUserRepository::new(sea_service.clone()).await);
    
    let fetch_many_users_service = FetchManyUsersService::new(
        user_repository,
    );

    fetch_many_users_service
}