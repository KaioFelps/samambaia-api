use async_trait::async_trait;
use std::error::Error;

use crate::domain::domain_entities::comment_report::{DraftCommentReport, CommentReport};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentReportRepositoryTrait {
    async fn create(&self, comment_report: DraftCommentReport) -> Result<CommentReport, Box<dyn Error>>;

    async fn find_by_id(&self, comm_report_id: i32) -> Result<Option<CommentReport>, Box<dyn Error>>;

    async fn save(&self, comment_report: CommentReport) -> Result<CommentReport, Box<dyn Error>>;
}