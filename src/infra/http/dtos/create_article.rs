use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateArticleDto {
    #[serde(rename = "authorId")]
    pub author_id: Option<Uuid>,

    pub content: String,

    #[validate(url(message = "Cover url must be a valid url."))]
    #[serde(rename = "coverUrl")]
    pub cover_url: String,

    pub title: String,
}