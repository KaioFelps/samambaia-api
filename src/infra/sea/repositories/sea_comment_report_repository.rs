use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use std::error::Error;

use crate::domain::domain_entities::comment_report::DraftCommentReport;
use crate::domain::repositories::comment_report_repository::CommentReportRepositoryTrait;
use crate::domain::domain_entities::comment_report::CommentReport;
use crate::infra::sea::mappers::sea_comment_report_mapper::SeaCommentReportMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaCommentReportRepository {
    sea_service: SeaService,
}

impl SeaCommentReportRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaCommentReportRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl CommentReportRepositoryTrait for SeaCommentReportRepository {
    async fn create(&self, comment_report: DraftCommentReport) -> Result<CommentReport, Box<dyn Error>> {
        let new_comment_report = SeaCommentReportMapper::comment_report_to_sea_active_model(comment_report);

        let db = &self.sea_service.db;

        let created_comment_report = new_comment_report.insert(db).await?;
        let created_comment_report = SeaCommentReportMapper::model_to_comment_report(created_comment_report);

        Ok(created_comment_report)
    }
}