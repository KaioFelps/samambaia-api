use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::{
    core::pagination::PaginationParameters, domain::domain_entities::announcement::Announcement,
};

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
        query: PaginationParameters<AnnouncementQueryType>,
    ) -> Result<FindManyAnnouncementsResponse, Box<dyn Error>>;
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn Error>>;
}
