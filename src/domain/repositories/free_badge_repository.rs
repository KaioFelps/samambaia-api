use async_trait::async_trait;
use std::error::Error;
use crate::domain::domain_entities::free_badge::FreeBadge;

#[cfg(test)]
use mockall::automock;
use crate::core::pagination::PaginationParameters;

#[derive(Debug)]
pub struct FindManyFreeBadgesResponse (
    pub Vec<FreeBadge>,
    pub u64,
);

#[cfg_attr(test, automock)]
#[async_trait]
pub trait FreeBadgeRepositoryTrait {
    async fn create(&self, free_badge: FreeBadge) -> Result<FreeBadge, Box<dyn Error>>;
    async fn save(&self, free_badge: FreeBadge) -> Result<FreeBadge, Box<dyn Error>>;
    async fn delete(&self, free_badge: FreeBadge) -> Result<(), Box<dyn Error>>;
    async fn find_many(&self, PaginationParameters<()>) -> Result<FindManyFreeBadgesResponse, Box<dyn Error>>;
}
