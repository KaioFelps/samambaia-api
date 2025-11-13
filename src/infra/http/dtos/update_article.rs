use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::libs::custom_deserializers::deserialize_optional_option;

#[derive(Deserialize, Validate)]
pub struct UpdateArticleDto {
    pub author_id: Option<Uuid>,

    pub content: Option<String>,

    pub description: Option<String>,

    #[validate(url(message = "Cover url must be a valid url."))]
    pub cover_url: Option<String>,

    pub title: Option<String>,

    pub approved: Option<bool>,

    pub tags: Option<Vec<i32>>,

    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub script: Option<Option<String>>,

    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub cleanup_script: Option<Option<String>>,
}
