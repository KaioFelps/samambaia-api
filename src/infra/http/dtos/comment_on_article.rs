use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CommentOnArticleDto {
    #[validate(
        required(message = "O comentário precisa ter conteúdo."),
        length(min = 1, message = "O comentário não pode estar vazio.")
    )]
    pub content: Option<String>,
}
