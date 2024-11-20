use entities::comment_report::ActiveModel as CommentReportActiveModel;
use entities::comment_report::Model as CommentReportModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::domain_entities::comment_report::CommentReportIdTrait;
use crate::domain::domain_entities::comment_report::CommentReportTrait;
use crate::domain::domain_entities::comment_report::DraftCommentReport;

pub struct SeaCommentReportMapper {}

impl SeaCommentReportMapper {
    pub fn comment_report_to_sea_model(comment_report: CommentReport) -> CommentReportModel {
        CommentReportModel {
            comment_id: comment_report.comment_id(),
            created_at: comment_report.created_at(),
            message: comment_report.message(),
            solved_by: comment_report.solved_by(),
            user_id: comment_report.user_id(),
            id: comment_report.id(),
        }
    }

    pub fn draft_comment_report_to_sea_active_model(
        comment_report: DraftCommentReport,
    ) -> CommentReportActiveModel {
        CommentReportActiveModel {
            user_id: comment_report.user_id().into_active_value(),
            comment_id: comment_report.comment_id().into_active_value(),
            message: comment_report.message().into_active_value(),
            solved_by: comment_report.solved_by().into_active_value(),
            created_at: comment_report.created_at().into_active_value(),
            ..Default::default()
        }
    }

    pub fn comment_report_to_sea_active_model(
        comment_report: CommentReport,
    ) -> CommentReportActiveModel {
        CommentReportActiveModel {
            user_id: comment_report.user_id().into_active_value(),
            comment_id: comment_report.comment_id().into_active_value(),
            message: comment_report.message().into_active_value(),
            solved_by: comment_report.solved_by().into_active_value(),
            created_at: comment_report.created_at().into_active_value(),
            id: comment_report.id().into_active_value(),
        }
    }

    pub fn active_model_to_comment_report(
        active_model_comment_report: CommentReportActiveModel,
    ) -> CommentReport {
        CommentReport::new_from_existing(
            active_model_comment_report.id.unwrap(),
            active_model_comment_report.comment_id.unwrap(),
            active_model_comment_report.user_id.unwrap(),
            active_model_comment_report.message.unwrap(),
            active_model_comment_report.solved_by.unwrap(),
            active_model_comment_report.created_at.unwrap(),
        )
    }

    pub fn model_to_comment_report(model_comment_report: CommentReportModel) -> CommentReport {
        CommentReport::new_from_existing(
            model_comment_report.id,
            model_comment_report.comment_id,
            model_comment_report.user_id,
            model_comment_report.message,
            model_comment_report.solved_by,
            model_comment_report.created_at,
        )
    }
}
