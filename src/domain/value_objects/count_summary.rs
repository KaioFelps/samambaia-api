use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, PartialEq, Eq, Debug, Serialize)]
pub struct CountSummary {
    pub users: i32,
    pub articles: i32,
    pub comments: i32,
    pub team_users: i32,
}
