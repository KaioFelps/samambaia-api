use entities::comment::Model as CommentModel;
use entities::comment::ActiveModel as CommentActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::comment::Comment;

pub struct SeaCommentMapper {}

impl SeaCommentMapper {
    pub fn comment_to_sea_model(comment: Comment) -> CommentModel {
        let sea_model = CommentModel {
            id: comment.id(),
            article_id: comment.article_id(),
            author_id: comment.author_id(),
            content: comment.content().into(),
            is_active: comment.is_active(),
            created_at: comment.created_at(),
        };

        sea_model
    }

    pub fn comment_to_sea_active_model(comment: Comment) -> CommentActiveModel {
        let sea_active_model = CommentActiveModel {
            id: comment.id().into_active_value(),
            article_id: comment.article_id().into_active_value(),
            author_id: comment.author_id().into_active_value(),
            content: comment.content().to_string().into_active_value(),
            is_active: comment.is_active().into_active_value(),
            created_at: comment.created_at().into_active_value(),
        };

        sea_active_model
    }

    pub fn active_model_to_comment(active_model_comment: CommentActiveModel) -> Comment {        
        let comment = Comment::new_from_existing(
            active_model_comment.id.unwrap(),
            active_model_comment.article_id.unwrap(),
            active_model_comment.author_id.unwrap(),
            active_model_comment.content.unwrap(),
            active_model_comment.is_active.unwrap(),
            active_model_comment.created_at.unwrap(),
        );

        comment
    }

    pub fn model_to_comment(model_comment: CommentModel) -> Comment {
        let comment = Comment::new_from_existing(
            model_comment.id.into(),
            model_comment.article_id.into(),
            model_comment.author_id.into(),
            model_comment.content.into(),
            model_comment.is_active.into(),
            model_comment.created_at.into(),
        );

        comment
    }
}