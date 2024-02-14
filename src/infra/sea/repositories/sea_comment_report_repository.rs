use async_trait::async_trait;
use sea_orm::{ColumnTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::error::Error;

use crate::domain::domain_entities::comment_report::CommentReportIdTrait;
use crate::domain::domain_entities::comment_report::DraftCommentReport;
use crate::domain::repositories::comment_report_repository::CommentReportRepositoryTrait;
use crate::domain::domain_entities::comment_report::CommentReport;
use crate::infra::sea::mappers::sea_comment_report_mapper::SeaCommentReportMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment_report::Entity as CommentReportEntity;
use entities::comment_report::Column as CommentReportColumn;

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
        let new_comment_report = SeaCommentReportMapper::draft_comment_report_to_sea_active_model(comment_report);

        let db = &self.sea_service.db;

        let created_comment_report = new_comment_report.insert(db).await?;
        let created_comment_report = SeaCommentReportMapper::model_to_comment_report(created_comment_report);

        Ok(created_comment_report)
    }

    async fn find_by_id(&self, comm_report_id: i32) -> Result<Option<CommentReport>, Box<dyn Error>> {
        let comm_report = CommentReportEntity::
        find_by_id(comm_report_id)
        .one(&self.sea_service.db)
        .await?;

        match comm_report {
            None => Ok(None),
            Some(comm_report) => {
                Ok( Some( SeaCommentReportMapper::model_to_comment_report(comm_report) ) )
            }
        }
    }

    async fn save(&self, comment_report: CommentReport) -> Result<CommentReport, Box<dyn Error>> {
        let comm_rep_id = comment_report.id();

        let comment_report = SeaCommentReportMapper::comment_report_to_sea_active_model(comment_report);

        let comment_report = CommentReportEntity
        ::update(comment_report)
        .filter(CommentReportColumn::Id.eq(comm_rep_id))
        .exec(&self.sea_service.db)
        .await?;

        let comment_report = SeaCommentReportMapper::model_to_comment_report(comment_report);

        Ok(comment_report)
    }
}