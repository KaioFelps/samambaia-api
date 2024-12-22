use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateArticleTagDto {
    #[validate(length(min = 1, message = "Article tag value must be at least one char long."))]
    pub value: Option<String>,
}
