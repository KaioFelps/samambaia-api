use async_trait::async_trait;
use entities::announcement::Column as AnnouncementColumn;
use entities::announcement::Entity as AnnouncementEntity;
use migration::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, EntityTrait, IntoSimpleExpr, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Select,
};
use std::error::Error;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::announcement::Announcement;
use crate::domain::repositories::announcements_repository::{
    AnnouncementQueryType, AnnouncementRepositoryTrait, FindManyAnnouncementsResponse,
};
use crate::infra::sea::mappers::{sea_announcement_mapper::SeaAnnouncementMapper, SeaMapper};
use crate::infra::sea::sea_service::SeaService;

pub struct SeaAnnouncementRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SeaAnnouncementRepository<'a> {
    pub async fn new(service: &'a SeaService) -> Self {
        SeaAnnouncementRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl AnnouncementRepositoryTrait for SeaAnnouncementRepository<'_> {
    async fn create(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>> {
        let announcement = SeaAnnouncementMapper::entity_into_active_model(announcement)
            .insert(&self.sea_service.db)
            .await?;

        Ok(SeaAnnouncementMapper::model_into_entity(announcement))
    }

    async fn save(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>> {
        let announcement = SeaAnnouncementMapper::entity_into_active_model(announcement)
            .update(&self.sea_service.db)
            .await?;

        Ok(SeaAnnouncementMapper::model_into_entity(announcement))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Announcement>, Box<dyn Error>> {
        Ok(AnnouncementEntity::find_by_id(*id)
            .one(&self.sea_service.db)
            .await?
            .map(SeaAnnouncementMapper::model_into_entity))
    }

    async fn find_many(
        &self,
        params: PaginationParameters<AnnouncementQueryType>,
    ) -> Result<FindManyAnnouncementsResponse, Box<dyn Error>> {
        let offset = ((params.page - 1) * params.items_per_page) as u64;

        let announcements = AnnouncementEntity::find()
            .order_by_desc(AnnouncementColumn::CreatedAt)
            .apply_if(
                params.query.clone(),
                SeaAnnouncementRepository::apply_query_filters,
            )
            .limit(params.items_per_page as u64)
            .offset(offset)
            .all(&self.sea_service.db)
            .await?
            .into_iter()
            .map(SeaAnnouncementMapper::model_into_entity)
            .collect::<Vec<Announcement>>();

        let count = AnnouncementEntity::find()
            .apply_if(params.query, SeaAnnouncementRepository::apply_query_filters)
            .count(&self.sea_service.db)
            .await?;

        Ok(FindManyAnnouncementsResponse(announcements, count))
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn Error>> {
        AnnouncementEntity::delete_by_id(*id)
            .exec(&self.sea_service.db)
            .await?;

        Ok(())
    }
}

impl SeaAnnouncementRepository<'_> {
    fn apply_query_filters(
        builder: Select<AnnouncementEntity>,
        query: AnnouncementQueryType,
    ) -> Select<AnnouncementEntity> {
        match query {
            AnnouncementQueryType::Description(description) => builder.filter(
                AnnouncementColumn::Description
                    .into_simple_expr()
                    .ilike(format!("%{}%", description)),
            ),
        }
    }
}
