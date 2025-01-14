use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::libs::time::TimeHelper;

use super::slug::Slug;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Article {
    id: Uuid,
    author_id: Uuid,
    cover_url: String,
    title: String,
    content: String,
    description: String,
    approved: bool,
    tag_id: Option<i32>,
    tag_value: Option<String>,
    created_at: DateTime,
    updated_at: Option<DateTime>,
    slug: Slug,
}

impl Article {
    // CONSTRUCTORS
    pub fn new(
        author_id: Uuid,
        title: String,
        content: String,
        cover_url: String,
        tag_id: i32,
        tag_value: String,
        description: String,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_at = TimeHelper::now();
        let updated_at = None;

        let slug = Slug::new(id, title.clone());

        Article {
            id,
            author_id,
            cover_url,
            title,
            content,
            tag_id: Some(tag_id),
            tag_value: Some(tag_value),
            approved: false,
            created_at,
            updated_at,
            slug,
            description,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_from_existing(
        id: Uuid,
        author_id: Uuid,
        cover_url: String,
        title: String,
        content: String,
        approved: bool,
        created_at: DateTime,
        updated_at: Option<DateTime>,
        tag_id: Option<i32>,
        tag_value: Option<String>,
        slug: Slug,
        description: String,
    ) -> Self {
        Article {
            id,
            author_id,
            cover_url,
            title,
            content,
            approved,
            tag_value,
            tag_id,
            created_at,
            updated_at,
            slug,
            description,
        }
    }

    // METHODS

    fn touch(&mut self) {
        self.updated_at = Some(TimeHelper::now());
    }

    // GETTERS

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn author_id(&self) -> Uuid {
        self.author_id
    }

    pub fn cover_url(&self) -> &str {
        self.cover_url.as_ref()
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    pub fn approved(&self) -> bool {
        self.approved
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<DateTime> {
        self.updated_at
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    pub fn tag_id(&self) -> Option<i32> {
        self.tag_id
    }

    pub fn tag_value(&self) -> &Option<String> {
        &self.tag_value
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    // SETTERS

    pub fn set_author_id(&mut self, author_id: Uuid) {
        self.author_id = author_id;
        self.touch();
    }

    pub fn set_cover_url(&mut self, cover_url: String) {
        self.cover_url = cover_url;
        self.touch();
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title.clone();
        self.slug = Slug::new(self.id, title);

        self.touch();
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.touch();
    }

    pub fn set_approved(&mut self, approved: bool) {
        self.approved = approved;
    }

    pub fn set_tag_id(&mut self, tag_id: i32) {
        self.tag_id = Some(tag_id);
    }

    pub fn set_tag_value(&mut self, tag_value: String) {
        self.tag_value = Some(tag_value);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.touch();
    }
}
