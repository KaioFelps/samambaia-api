use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateArticleTagDto {
    #[validate(length(min=1, message = "Article tag can't be empty."))]
    pub value: String
}
