use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

use super::article_tag::ArticleTag;
use super::slug::Slug;

#[derive(Deserialize)]
pub struct ArticlePreviewAuthor {
    nickname: String,
    id: Uuid,
}

impl ArticlePreviewAuthor {
    pub fn new(id: Uuid, nickname: String) -> Self {
        Self { nickname, id }
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

#[derive(Deserialize)]
pub struct ArticlePreview {
    id: Uuid,
    cover_url: String,
    title: String,
    description: String,
    approved: bool,
    tags: Vec<ArticleTag>,
    author: ArticlePreviewAuthor,
    created_at: NaiveDateTime,
    slug: Slug,
}

impl ArticlePreview {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        cover_url: String,
        title: String,
        description: String,
        approved: bool,
        tags: Vec<ArticleTag>,
        author: ArticlePreviewAuthor,
        created_at: NaiveDateTime,
        slug: Slug,
    ) -> Self {
        Self {
            id,
            cover_url,
            title,
            description,
            approved,
            tags,
            author,
            created_at,
            slug,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn cover_url(&self) -> &str {
        &self.cover_url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn approved(&self) -> bool {
        self.approved
    }

    pub fn tags(&self) -> &[ArticleTag] {
        self.tags.iter().as_slice()
    }

    pub fn author(&self) -> &ArticlePreviewAuthor {
        &self.author
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}
