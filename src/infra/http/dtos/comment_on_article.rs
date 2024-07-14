use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CommentOnArticleDto {
    #[validate(length(min=1, message = "Comment can't be empty."))]
    pub content: String
}
