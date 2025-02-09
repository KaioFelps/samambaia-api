use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::announcement::Announcement;

#[derive(Clone)]
pub enum AnnouncementQueryType {
    Description(String),
}

pub struct FindManyAnnouncementsResponse(pub Vec<Announcement>, pub u64);

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnnouncementRepositoryTrait {
    async fn create(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>>;
    async fn save(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Announcement>, Box<dyn Error>>;
    async fn find_many(
        &self,
        params: PaginationParameters<AnnouncementQueryType>,
    ) -> Result<FindManyAnnouncementsResponse, Box<dyn Error>>;
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn Error>>;
}
