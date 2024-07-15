use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateCommentReportDto {
    #[validate(length(min=1, message = "Comment report can't be empty."))]
    pub content: String
}