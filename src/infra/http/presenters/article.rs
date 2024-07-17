use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize, Deserialize)]
pub struct MappedArticle {
    id: Uuid,
    #[serde(rename = "authorId")]
    author_id: Uuid,
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
}

pub struct ArticlePresenter;

impl PresenterTrait<Article, MappedArticle> for ArticlePresenter {
    fn to_http(article: Article) -> MappedArticle {
        MappedArticle {
            id: article.id(),
            author_id: article.author_id(),
            title: article.title().into(),
            cover_url: article.cover_url().into(),
            content: article.content().into(),
            slug: article.slug().to_string(),
            approved: article.approved(),
            created_at: article.created_at(),
            updated_at: article.updated_at(),
        }
    }
}
