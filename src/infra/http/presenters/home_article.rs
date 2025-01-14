use chrono::NaiveDateTime as DateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    domain::domain_entities::home_article::HomeArticle,
    infra::http::presenters::presenter::PresenterTrait,
};

#[derive(Serialize)]
pub struct MappedHomeArticleAuthor {
    pub id: Uuid,
    pub nickname: String,
}

#[derive(Serialize)]
pub struct MappedHomeArticleTag {
    pub id: i32,
    pub value: String,
}

#[derive(Serialize)]
pub struct MappedHomeArticle {
    pub id: Uuid,
    pub author: MappedHomeArticleAuthor,
    #[serde(rename = "coverUrl")]
    pub cover_url: String,
    pub title: String,
    pub tag: Option<MappedHomeArticleTag>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    pub slug: String,
}

pub struct HomeArticlePresenter;

impl PresenterTrait<HomeArticle, MappedHomeArticle> for HomeArticlePresenter {
    fn to_http((article, user): HomeArticle) -> MappedHomeArticle {
        let tag = article.tag_id().map(|id| MappedHomeArticleTag {
            id,
            value: article.tag_value().unwrap(),
        });

        MappedHomeArticle {
            author: MappedHomeArticleAuthor {
                id: user.id(),
                nickname: user.nickname().into(),
            },
            cover_url: article.cover_url().into(),
            created_at: article.created_at(),
            id: article.id(),
            slug: article.slug().to_string(),
            tag,
            title: article.title().into(),
        }
    }
}
