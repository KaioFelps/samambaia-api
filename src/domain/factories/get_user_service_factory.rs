use crate::domain::services::get_user_service::GetUserService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> GetUserService<SeaUserRepository> {
    let user_repository = SeaUserRepository::new(db_conn);
    GetUserService::new(user_repository)
}
