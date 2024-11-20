use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::comment_report::{CommentReport, DraftCommentReport};

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyCommentReportsResponse(pub Vec<CommentReport>, pub u64);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CommentReportQueryType {
    SolvedBy(Uuid),
    Solved(bool),
    Content(String),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentReportRepositoryTrait {
    async fn create(
        &self,
        comment_report: DraftCommentReport,
    ) -> Result<CommentReport, Box<dyn Error>>;

    async fn find_by_id(
        &self,
        comm_report_id: i32,
    ) -> Result<Option<CommentReport>, Box<dyn Error>>;

    async fn find_many(
        &self,
        params: PaginationParameters<CommentReportQueryType>,
    ) -> Result<FindManyCommentReportsResponse, Box<dyn Error>>;

    async fn save(&self, comment_report: CommentReport) -> Result<CommentReport, Box<dyn Error>>;

    async fn delete(&self, comment_report: CommentReport) -> Result<(), Box<dyn Error>>;
}
