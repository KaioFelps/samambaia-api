use entities::comment_report::Model as CommentReportModel;
use entities::comment_report::ActiveModel as CommentReportActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::domain_entities::comment_report::CommentReportIdTrait;
use crate::domain::domain_entities::comment_report::CommentReportTrait;
use crate::domain::domain_entities::comment_report::DraftCommentReport;

pub struct SeaCommentReportMapper {}

impl SeaCommentReportMapper {
    pub fn comment_report_to_sea_model(comment_report: CommentReport) -> CommentReportModel {
        let sea_model = CommentReportModel {
            comment_id: comment_report.comment_id(),
            created_at: comment_report.created_at(),
            message: comment_report.message(),
            solved_by: comment_report.solved_by(),
            user_id: comment_report.user_id(),
            id: comment_report.id()
        };

        sea_model
    }

    pub fn draft_comment_report_to_sea_active_model(comment_report: DraftCommentReport) -> CommentReportActiveModel {
        let sea_active_model = CommentReportActiveModel {
            user_id: comment_report.user_id().into_active_value(),
            comment_id: comment_report.comment_id().into_active_value(),
            message: comment_report.message().into_active_value(),
            solved_by: comment_report.solved_by().into_active_value(),
            created_at: comment_report.created_at().into_active_value(),
            ..Default::default()
        };

        sea_active_model
    }

    pub fn comment_report_to_sea_active_model(comment_report: CommentReport) -> CommentReportActiveModel {
        let sea_active_model = CommentReportActiveModel {
            user_id: comment_report.user_id().into_active_value(),
            comment_id: comment_report.comment_id().into_active_value(),
            message: comment_report.message().into_active_value(),
            solved_by: comment_report.solved_by().into_active_value(),
            created_at: comment_report.created_at().into_active_value(),
            id: comment_report.id().into_active_value(),
        };

        sea_active_model
    }

    pub fn active_model_to_comment_report(active_model_comment_report: CommentReportActiveModel) -> CommentReport {        
        let comment_report = CommentReport::new_from_existing(
            active_model_comment_report.id.unwrap(),
            active_model_comment_report.comment_id.unwrap(),
            active_model_comment_report.user_id.unwrap(),
            active_model_comment_report.message.unwrap(),
            active_model_comment_report.solved_by.unwrap(),
            active_model_comment_report.created_at.unwrap(),
        );

        comment_report
    }

    pub fn model_to_comment_report(model_comment_report: CommentReportModel) -> CommentReport {
        let comment_report = CommentReport::new_from_existing(
            model_comment_report.id.into(),
            model_comment_report.comment_id.into(),
            model_comment_report.user_id.into(),
            model_comment_report.message.into(),
            model_comment_report.solved_by.into(),
            model_comment_report.created_at.into(),
        );

        comment_report
    }
}