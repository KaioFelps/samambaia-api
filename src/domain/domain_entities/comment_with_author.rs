use chrono::{NaiveDateTime as DateTime, Utc};
use uuid::Uuid;

use super::user::User;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommentWithAuthor {
    id: Uuid,
    article_id: Uuid,
    content: String,
    created_at: DateTime,
    author: User
}

impl CommentWithAuthor{
    // CONSTRUCTORS
    pub fn new(
        article_id: Uuid,
        content: String,
        author: User,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_at  = Utc::now().naive_utc();

        CommentWithAuthor {
            id,
            article_id,
            content,
            created_at,
            author
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        article_id: Uuid,
        content: String,
        created_at: DateTime,
        author: User
    ) -> Self {
        CommentWithAuthor {
            article_id,
            content,
            created_at,
            id,
            author
        }
    }

    // GETTERS
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn article_id(&self) -> Uuid {
        self.article_id
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    // SETTERS
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}
