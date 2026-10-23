use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct ShowArticleQueryDto {
    #[serde(rename = "commentsPage")]
    pub comments_page: Option<u32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AdminListAuditLogsDto {
    pub page: Option<usize>,
    #[validate(length(max = 200))]
    pub search: Option<String>,
}
