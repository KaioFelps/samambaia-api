use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateArticleDto {
    pub author_id: Option<Uuid>,

    #[validate(required(message = "O corpo da notícia é obrigatório."))]
    pub content: Option<String>,

    #[validate(required(message = "A descrição é obrigatória."))]
    pub description: Option<String>,

    #[validate(
        required(message = "O topstory é obrigatório."),
        url(message = "O topstory precisa ser um link válido.")
    )]
    pub cover_url: Option<String>,

    #[validate(required(message = "O título da notícia é obrigatório."))]
    pub title: Option<String>,

    pub tags: Vec<i32>,

    pub script: Option<String>,

    pub cleanup_script: Option<String>,
}
