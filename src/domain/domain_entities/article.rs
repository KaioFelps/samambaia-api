use chrono::{NaiveDateTime as DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Article {
    id: Uuid,
    author_id: Uuid,
    cover_url: String,
    title: String,
    content: String,
    approved: bool,
    created_at: DateTime,
    updated_at: Option<DateTime>,
}

impl Article {

    // CONSTRUCTORS
    pub fn new(
        author_id: Uuid,
        title: String,
        content: String,
        cover_url: String
    ) -> Self {
        let id = Uuid::new_v4();

        let created_at  = Utc::now().naive_utc();
        let updated_at = None;

        Article {
            id,
            author_id,
            cover_url,
            title,
            content,
            approved: false,
            created_at,
            updated_at,
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        author_id: Uuid,
        cover_url: String,
        title: String,
        content: String,
        approved: bool,
        created_at: DateTime,
        updated_at: Option<DateTime>,
    ) -> Self {
        Article {
            id,
            author_id,
            cover_url,
            title,
            content,
            approved,
            created_at,
            updated_at,
        }
    }

    // METHODS

    fn touch(&mut self) {
        self.updated_at = Some(Utc::now().naive_utc());
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
        self.title = title;
        self.touch();
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.touch();
    }

    pub fn set_approved(&mut self, approved: bool) {
        self.approved = approved;
    }
}