use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::domain::domain_entities::council_alert::{CouncilAlert, CouncilAlertDraft};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CouncilAlertRepositoryTrait {
    async fn create(
        &self,
        council_alert: CouncilAlertDraft,
    ) -> Result<CouncilAlert, Box<dyn Error>>;

    async fn find_all(&self) -> Result<Vec<CouncilAlert>, Box<dyn Error>>;
}
