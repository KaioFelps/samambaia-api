use std::future::Future;

pub trait ArticleRepository {
    // async fn find_by_id(&self, id: String) -> Result<Option<entities::article::Model>, sea_orm::DbErr>;
    fn find_by_id(&self, id: String) -> impl Future<Output = Result<Option<entities::article::Model>, sea_orm::DbErr>> + Send;
    fn find_all(&self) -> impl Future<Output = Result<Vec<entities::article::Model>, sea_orm::DbErr>>;
}