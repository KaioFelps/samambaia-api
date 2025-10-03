use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShowArticleQueryDto {
    #[serde(rename = "commentsPage")]
    pub comments_page: Option<u32>,
}
