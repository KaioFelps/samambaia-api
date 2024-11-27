use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::domain::domain_entities::announcement::Announcement;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnnouncementRepositoryTrait {
    async fn create(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>>;
    async fn save(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Announcement>, Box<dyn Error>>;
}
