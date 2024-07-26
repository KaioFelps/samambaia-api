use std::error::Error;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait,QueryOrder, QuerySelect};
use crate::core::pagination::PaginationParameters;
use crate::infra::sea::sea_service::SeaService;
use entities::free_badge::Column as FreeBadgeColumn;
use entities::free_badge::Entity as FreeBadgeEntity;
use crate::domain::domain_entities::free_badge::FreeBadge;
use crate::domain::repositories::free_badge_repository::{FindManyFreeBadgesResponse, FreeBadgeRepositoryTrait};
use crate::infra::sea::mappers::sea_free_badge_mapper::SeaFreeBadgeMapper;

pub struct SeaFreeBadgeRepository {
    sea_service: SeaService,
}

impl SeaFreeBadgeRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaFreeBadgeRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl FreeBadgeRepositoryTrait for SeaFreeBadgeRepository {
    async fn create(&self, free_badge: FreeBadge) -> Result<FreeBadge, Box<dyn Error>> {
        let free_badge = SeaFreeBadgeMapper::free_badge_to_sea_active_model(free_badge);
        let free_badge = free_badge.insert(&self.sea_service.db).await?;
        let free_badge = SeaFreeBadgeMapper::model_to_free_badge(free_badge);

        Ok(free_badge)
    }

    async fn save(&self, free_badge: FreeBadge) -> Result<FreeBadge, Box<dyn Error>> {
        let free_badge = SeaFreeBadgeMapper::free_badge_to_sea_active_model(free_badge);
        let free_badge = free_badge.update(&self.sea_service.db).await?;
        let free_badge = SeaFreeBadgeMapper::model_to_free_badge(free_badge);
        Ok(free_badge)
    }

    async fn delete(&self, free_badge: FreeBadge) -> Result<(), Box<dyn Error>> {
        let free_badge = SeaFreeBadgeMapper::free_badge_to_sea_active_model(free_badge);
        free_badge.delete(&self.sea_service.db).await?;

        Ok(())
    }

    async fn find_many(&self, params: PaginationParameters<()>) -> Result<FindManyFreeBadgesResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let badges = FreeBadgeEntity::find()
            .order_by_desc(FreeBadgeColumn::CreatedAt)
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let badges_count = FreeBadgeEntity::find()
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mapped_badges: Vec<FreeBadge> = badges.into_iter().map(SeaFreeBadgeMapper::model_to_free_badge).collect();

        Ok(FindManyFreeBadgesResponse (mapped_badges, badges_count))
    }
}
