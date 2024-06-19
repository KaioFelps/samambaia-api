use crate::domain::domain_entities::{comment_with_author::CommentWithAuthor, role::Role};
use chrono::NaiveDateTime as DateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MappedComment {
    id: Uuid,
    content: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    #[serde(rename = "author.nickname")]
    author_nickname: String,
    #[serde(rename = "author.role")]
    author_role: Role
}


pub struct CommentPresenter;

impl CommentPresenter {
    pub fn to_http(comment: CommentWithAuthor) -> MappedComment {
        let author = comment.author();
        MappedComment {
            id: comment.id(),
            author_nickname: author.nickname().to_owned(),
            author_role: author.role().unwrap(),
            content: comment.content().to_owned(),
            created_at: comment.created_at()
        }
    }
}
