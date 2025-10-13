use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateCommentReportDto {
    #[validate(
        required(message = "Comment report needs a reason/description."),
        length(min = 1, message = "Comment report can't be empty.")
    )]
    pub content: Option<String>,
}
