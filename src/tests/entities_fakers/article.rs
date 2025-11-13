use chrono::NaiveDateTime as DateTime;
use derive_builder::Builder;
use fake::{Fake, Faker, faker};
use faker::lorem::pt_br::{Paragraphs, Sentence};
use url::Url;
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::value_objects::slug::Slug;
use crate::libs::time::TimeHelper;

#[derive(Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ArticleFaker {
    id: Option<Uuid>,
    author_id: Uuid,
    cover_url: Option<String>,
    title: Option<String>,
    content: Option<String>,
    description: Option<String>,
    approved: Option<bool>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
    slug: Option<Slug>,
    tags: Option<Vec<ArticleTag>>,
    script: Option<String>,
    cleanup_script: Option<String>,
}

impl From<ArticleFaker> for Article {
    fn from(value: ArticleFaker) -> Self {
        let id = value.id.unwrap_or_else(Uuid::new_v4);
        let title = value.title.unwrap_or_else(|| Sentence(1..5).fake());
        Article::new_from_existing(
            id,
            value.author_id,
            value
                .cover_url
                .unwrap_or_else(|| Faker.fake::<Url>().into()),
            title.clone(),
            value.content.unwrap_or_else(|| {
                Paragraphs(2..5)
                    .fake::<Vec<String>>()
                    .into_iter()
                    .map(|p| format!("<p>{p}</p>"))
                    .collect::<Vec<String>>()
                    .join("\n")
            }),
            value.approved.unwrap_or(false),
            value.created_at.unwrap_or_else(TimeHelper::now),
            value.updated_at,
            value.slug.unwrap_or_else(|| Slug::new(id, title)),
            value.description.unwrap_or_else(|| Sentence(1..4).fake()),
            value.tags.unwrap_or_default(),
            value.script,
            value.cleanup_script,
        )
    }
}

impl ArticleFaker {
    pub fn into_entity(self) -> Article {
        self.into()
    }
}
