use crate::domain::domain_entities::comment_report::{
    CommentReport, CommentReportIdTrait, CommentReportTrait,
};
use crate::domain::domain_entities::role::Role;
use crate::infra::http::presenters::presenter::PresenterTrait;
use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct MappedCommentAuthor {
    nickname: String,
    role: Role,
}

#[derive(Serialize, Deserialize)]
pub struct MappedCommentReport {
    id: i32,
    #[serde(rename = "commentId")]
    comment_id: Uuid,
    #[serde(rename = "userId")]
    user_id: Uuid,
    message: String,
    #[serde(rename = "solvedBy")]
    solved_by: Option<Uuid>,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
}

pub struct CommentReportPresenter;

impl PresenterTrait<CommentReport, MappedCommentReport> for CommentReportPresenter {
    fn to_http(report: CommentReport) -> MappedCommentReport {
        MappedCommentReport {
            id: report.id(),
            user_id: report.user_id(),
            solved_by: report.solved_by(),
            comment_id: report.comment_id(),
            message: report.message(),
            created_at: report.created_at(),
        }
    }
}
