use crate::domain::domain_entities::comment::Comment;
use crate::domain::domain_entities::{comment_with_author::CommentWithAuthor, role::Role};
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
pub struct MappedComment {
    id: Uuid,
    content: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    author: MappedCommentAuthor,
}

#[derive(Serialize, Deserialize)]
pub struct MappedRawComment {
    id: Uuid,
    content: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    #[serde(rename = "isActive")]
    is_active: bool,
    author_id: Uuid,
}

pub struct CommentPresenter;

impl PresenterTrait<CommentWithAuthor, MappedComment> for CommentPresenter {
    fn to_http(comment: CommentWithAuthor) -> MappedComment {
        let author = comment.author();
        MappedComment {
            id: comment.id(),
            content: comment.content().to_owned(),
            created_at: comment.created_at(),
            author: MappedCommentAuthor {
                nickname: author.nickname().to_owned(),
                role: author.role().unwrap(),
            },
        }
    }
}

impl CommentPresenter {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_http_raw(comment: Comment) -> MappedRawComment {
        MappedRawComment {
            id: comment.id(),
            author_id: comment.author_id(),
            content: comment.content().to_owned(),
            is_active: comment.is_active(),
            created_at: comment.created_at(),
        }
    }
}
