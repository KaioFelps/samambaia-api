use std::future::Future;

pub struct CreateUserParam {
    pub nickname: String,
    pub password: String,
}

pub trait UserRepository {
    // async fn find_by_id(&self, id: String) -> Result<Option<entities::article::Model>, sea_orm::DbErr>;
    fn create(&self, params: CreateUserParam) -> impl Future<Output = Result<entities::user::Model, sea_orm::DbErr>>;
    fn find_all(&self) -> impl Future<Output = Result<Vec<entities::user::Model>, sea_orm::DbErr>>;
}