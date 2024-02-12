use chrono::{NaiveDateTime as DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comment {
    id: Uuid,
    article_id: Uuid,
    author_id: Uuid,
    content: String,
    created_at: DateTime,
}

impl Comment {

    // CONSTRUCTORS
    pub fn new(
        author_id: Uuid,
        article_id: Uuid,
        content: String,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_at  = Utc::now().naive_utc();

        Comment {
            id,
            article_id,
            author_id,
            content,
            created_at
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        article_id: Uuid,
        author_id: Uuid,
        content: String,
        created_at: DateTime,
    ) -> Self {
        Comment {
            article_id,
            author_id,
            content,
            created_at,
            id
        }
    }

    // GETTERS

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn article_id(&self) -> Uuid {
        self.article_id
    }

    pub fn author_id(&self) -> Uuid {
        self.author_id
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    // SETTERS

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}