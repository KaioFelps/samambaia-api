use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::libs::time::TimeHelper;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comment {
    id: Uuid,
    article_id: Option<Uuid>,
    author_id: Uuid,
    content: String,
    is_active: bool,
    created_at: DateTime,
}

impl Comment {
    // CONSTRUCTORS
    pub fn new(author_id: Uuid, article_id: Option<Uuid>, content: String) -> Self {
        let id = Uuid::new_v4();
        let created_at = TimeHelper::now();
        let is_active = true;

        Comment {
            id,
            article_id,
            author_id,
            content,
            is_active,
            created_at,
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        article_id: Option<Uuid>,
        author_id: Uuid,
        content: String,
        is_active: bool,
        created_at: DateTime,
    ) -> Self {
        Comment {
            id,
            article_id,
            author_id,
            content,
            is_active,
            created_at,
        }
    }

    // GETTERS

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn article_id(&self) -> Option<Uuid> {
        self.article_id
    }

    pub fn author_id(&self) -> Uuid {
        self.author_id
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    // SETTERS

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active
    }
}
