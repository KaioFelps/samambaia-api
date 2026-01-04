use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateArticleTagDto {
    #[validate(length(
        min = 1,
        message = "O conteúdo de uma tag de notícia precisa ter pelo menos 1 caractere."
    ))]
    pub value: Option<String>,
}
