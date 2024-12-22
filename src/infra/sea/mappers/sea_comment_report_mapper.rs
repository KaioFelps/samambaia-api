use entities::comment_report::ActiveModel as CommentReportActiveModel;
use entities::comment_report::Model as CommentReportModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::domain_entities::comment_report::CommentReportIdTrait;
use crate::domain::domain_entities::comment_report::CommentReportTrait;
use crate::domain::domain_entities::comment_report::DraftCommentReport;

use super::SeaMapper;

pub struct SeaCommentReportMapper;

impl SeaMapper<CommentReport, CommentReportModel, CommentReportActiveModel>
    for SeaCommentReportMapper
{
    fn entity_into_model(entity: CommentReport) -> CommentReportModel {
        CommentReportModel {
            comment_id: entity.comment_id(),
            created_at: entity.created_at(),
            message: entity.message(),
            solved_by: entity.solved_by(),
            user_id: entity.user_id(),
            id: entity.id(),
        }
    }

    fn entity_into_active_model(entity: CommentReport) -> CommentReportActiveModel {
        CommentReportActiveModel {
            user_id: entity.user_id().into_active_value(),
            comment_id: entity.comment_id().into_active_value(),
            message: entity.message().into_active_value(),
            solved_by: entity.solved_by().into_active_value(),
            created_at: entity.created_at().into_active_value(),
            id: entity.id().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: CommentReportActiveModel) -> CommentReport {
        CommentReport::new_from_existing(
            active_model.id.unwrap(),
            active_model.comment_id.unwrap(),
            active_model.user_id.unwrap(),
            active_model.message.unwrap(),
            active_model.solved_by.unwrap(),
            active_model.created_at.unwrap(),
        )
    }

    fn model_into_entity(model: CommentReportModel) -> CommentReport {
        CommentReport::new_from_existing(
            model.id,
            model.comment_id,
            model.user_id,
            model.message,
            model.solved_by,
            model.created_at,
        )
    }
}

impl SeaCommentReportMapper {
    pub fn draft_entity_into_active_model(
        draft_entity: DraftCommentReport,
    ) -> CommentReportActiveModel {
        CommentReportActiveModel {
            user_id: draft_entity.user_id().into_active_value(),
            comment_id: draft_entity.comment_id().into_active_value(),
            message: draft_entity.message().into_active_value(),
            solved_by: draft_entity.solved_by().into_active_value(),
            created_at: draft_entity.created_at().into_active_value(),
            ..Default::default()
        }
    }
}
