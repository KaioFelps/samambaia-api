use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateArticleDto {
    #[serde(rename = "authorId")]
    pub author_id: Option<Uuid>,

    pub content: Option<String>,

    #[validate(url(message = "Cover url must be a valid url."))]
    #[serde(rename = "coverUrl")]
    pub cover_url: Option<String>,

    pub title: Option<String>,

    pub approved: Option<bool>,
}