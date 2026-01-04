use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateArticleTagDto {
    #[validate(length(min = 1, message = "Uma tag não pode ser um texto vazio."))]
    pub value: String,
}
