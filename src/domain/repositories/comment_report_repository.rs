use async_trait::async_trait;
use std::error::Error;

use crate::domain::domain_entities::comment_report::{DraftCommentReport, CommentReport};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentReportRepositoryTrait {
    async fn create(&self, comment_report: DraftCommentReport) -> Result<CommentReport, Box<dyn Error>>;
}