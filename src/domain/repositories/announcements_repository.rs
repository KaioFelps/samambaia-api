use async_trait::async_trait;
use std::error::Error;

#[cfg(test)]
use mockall::automock;

use crate::domain::domain_entities::announcement::Announcement;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AnnouncementRepositoryTrait {
    async fn create(&self, announcement: Announcement) -> Result<Announcement, Box<dyn Error>>;
}
