use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateArticleDto {
    pub author_id: Option<Uuid>,

    pub content: String,

    pub description: String,

    #[validate(url(message = "Cover url must be a valid url."))]
    pub cover_url: String,

    pub title: String,

    pub tags: Vec<i32>,

    pub script: Option<String>,
}
