use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::domain::value_objects::count_summary::CountSummary;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SummaryRepositoryTrait {
    async fn get_table_summary(&self) -> Result<CountSummary, Box<dyn Error>>;
}
