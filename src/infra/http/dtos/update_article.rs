use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateArticleDto {
    pub author_id: Option<Uuid>,

    pub content: Option<String>,

    pub description: Option<String>,

    #[validate(url(message = "Cover url must be a valid url."))]
    pub cover_url: Option<String>,

    pub title: Option<String>,

    pub approved: Option<bool>,

    pub tag_id: Option<i32>,
}
