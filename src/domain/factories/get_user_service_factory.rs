use crate::domain::services::get_user_service::GetUserService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec(db_conn: &SeaService) -> GetUserService<SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn).await);
    GetUserService::new(user_repository)
}
