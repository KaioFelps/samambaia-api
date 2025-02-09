use entities::comment::Model as CommentModel;
use entities::user::Model as UserModel;

use super::sea_user_mapper::SeaUserMapper;
use super::SeaMapper;
use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;

pub struct SeaCommentWithAuthorMapper;

impl SeaCommentWithAuthorMapper {
    pub fn models_into_entity(entities: (CommentModel, UserModel)) -> CommentWithAuthor {
        let (comment_model, user_model) = entities;

        CommentWithAuthor::new_from_existing(
            comment_model.id,
            comment_model.article_id,
            comment_model.content,
            comment_model.is_active,
            comment_model.created_at,
            SeaUserMapper::model_into_entity(user_model),
        )
    }
}
