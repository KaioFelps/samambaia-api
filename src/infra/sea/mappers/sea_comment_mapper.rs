use entities::comment::ActiveModel as CommentActiveModel;
use entities::comment::Model as CommentModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::comment::Comment;

use super::SeaMapper;

pub struct SeaCommentMapper;

impl SeaMapper<Comment, CommentModel, CommentActiveModel> for SeaCommentMapper {
    fn entity_into_model(entity: Comment) -> CommentModel {
        CommentModel {
            id: entity.id(),
            article_id: entity.article_id(),
            author_id: entity.author_id(),
            content: entity.content().into(),
            is_active: entity.is_active(),
            created_at: entity.created_at(),
        }
    }

    fn entity_into_active_model(entity: Comment) -> CommentActiveModel {
        CommentActiveModel {
            id: entity.id().into_active_value(),
            article_id: entity.article_id().into_active_value(),
            author_id: entity.author_id().into_active_value(),
            content: entity.content().to_string().into_active_value(),
            is_active: entity.is_active().into_active_value(),
            created_at: entity.created_at().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: CommentActiveModel) -> Comment {
        Comment::new_from_existing(
            active_model.id.unwrap(),
            active_model.article_id.unwrap(),
            active_model.author_id.unwrap(),
            active_model.content.unwrap(),
            active_model.is_active.unwrap(),
            active_model.created_at.unwrap(),
        )
    }

    fn model_into_entity(model: CommentModel) -> Comment {
        Comment::new_from_existing(
            model.id,
            model.article_id,
            model.author_id,
            model.content,
            model.is_active,
            model.created_at,
        )
    }
}
