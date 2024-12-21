use crate::domain::services::fetch_many_users_service::FetchManyUsersService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::infra::sea::sea_service::SeaService;

pub fn exec(db_conn: &SeaService) -> FetchManyUsersService<SeaUserRepository> {
    let user_repository = Box::new(SeaUserRepository::new(db_conn));
    FetchManyUsersService::new(user_repository)
}
