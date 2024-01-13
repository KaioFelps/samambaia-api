use entities::article;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use crate::{repositories::article_repository::ArticleRepository, infra::sea::sea_service::SeaService};

pub struct SeaArticleRepository {
    // db_conn: DatabaseConnection,
    sea_service: SeaService
}

impl SeaArticleRepository {
    // constructor
    pub async fn new(service: &SeaService) -> Self {
        SeaArticleRepository {
            sea_service: service.clone()
        }
    }
}

impl ArticleRepository for SeaArticleRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<entities::article::Model>, sea_orm::DbErr> {
        entities::article::Entity::find().filter(article::Column::Id.eq(id)).one(&self.sea_service.db).await
    }

    async fn find_all(&self) -> Result<Vec<entities::article::Model>, sea_orm::DbErr> {
        entities::article::Entity::find().all(&self.sea_service.db).await
    }
}