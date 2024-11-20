use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;

use entities::comment::Model as CommentModel;
use entities::user::Model as UserModel;

use super::sea_user_mapper::SeaUserMapper;

pub struct SeaCommentWithAuthorMapper {}

impl SeaCommentWithAuthorMapper {
    pub fn model_to_comment_with_author(models: (CommentModel, UserModel)) -> CommentWithAuthor {
        let (model_comment, model_user) = models;

        let domain_author = SeaUserMapper::model_to_user(model_user);

        CommentWithAuthor::new_from_existing(
            model_comment.id,
            model_comment.article_id,
            model_comment.content,
            model_comment.is_active,
            model_comment.created_at,
            domain_author,
        )
    }
}
