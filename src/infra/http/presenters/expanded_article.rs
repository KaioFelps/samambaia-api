use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;
use crate::{core::pagination::PaginationResponse, domain::domain_entities::{article::Article, comment_with_author::CommentWithAuthor, user::User}};
use crate::infra::http::presenters::presenter::PresenterTrait;
use super::{comment::{CommentPresenter, MappedComment}, pagination::{MappedPagination, PaginationPresenter}, user::{MappedUser, UserPresenter}};

#[derive(Serialize, Deserialize)]
pub struct MappedExpandedArticle {
    id: Uuid,
    #[serde(rename = "coverUrl")]
    cover_url: String,
    title: String,
    content: String,
    approved: bool,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    #[serde(rename = "updatedAt")]
    updated_at: Option<DateTime>,
    slug: String,

    author: MappedUser,

    comments: MappedExpandedArticleComments
}

#[derive(Serialize, Deserialize)]
struct MappedExpandedArticleComments {
    data: Vec<MappedComment>,
    pagination: MappedPagination
}

pub struct ExpandedArticlePresenter;

impl ExpandedArticlePresenter {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_http(article: Article, author: User, comments: Vec<CommentWithAuthor>, pagination: (PaginationResponse, u8)) -> MappedExpandedArticle {
        MappedExpandedArticle {
            id: article.id(),
            title: article.title().into(),
            cover_url: article.cover_url().into(),
            content: article.content().into(),
            slug: article.slug().to_string(),
            approved: article.approved(),
            created_at: article.created_at(),
            updated_at: article.updated_at(),

            author: UserPresenter::to_http(author),

            comments: MappedExpandedArticleComments {
                data: comments.into_iter().map(CommentPresenter::to_http).collect(),
                pagination: PaginationPresenter::to_http(pagination.0, pagination.1)
            }
        }
    }
}
