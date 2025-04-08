use chrono::NaiveDateTime as DateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::domain_entities::article_preview::ArticlePreview;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize)]
pub struct MappedArticlePreviewAuthor {
    pub id: Uuid,
    pub nickname: String,
}

#[derive(Serialize)]
pub struct MappedArticlePreviewTag {
    pub id: i32,
    pub value: String,
}

#[derive(Serialize)]
pub struct MappedArticlePreview {
    pub id: Uuid,
    pub author: MappedArticlePreviewAuthor,
    #[serde(rename = "coverUrl")]
    pub cover_url: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<MappedArticlePreviewTag>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    pub slug: String,
    pub approved: bool,
}

pub struct ArticlePreviewPresenter;

impl PresenterTrait<ArticlePreview, MappedArticlePreview> for ArticlePreviewPresenter {
    fn to_http(article: ArticlePreview) -> MappedArticlePreview {
        let tags = article
            .tags()
            .iter()
            .map(|tag| MappedArticlePreviewTag {
                id: tag.id(),
                value: tag.value().to_owned(),
            })
            .collect::<Vec<_>>();

        let author = MappedArticlePreviewAuthor {
            id: *article.author().id(),
            nickname: article.author().nickname().to_owned(),
        };

        MappedArticlePreview {
            author,
            id: article.id().to_owned(),
            tags,
            slug: article.slug().to_string(),
            title: article.title().into(),
            cover_url: article.cover_url().into(),
            created_at: article.created_at(),
            description: article.description().into(),
            approved: article.approved(),
        }
    }
}
