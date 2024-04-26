use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::libs::time::TimeHelper;

use super::user::User;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommentWithAuthor {
    id: Uuid,
    article_id: Option<Uuid>,
    content: String,
    is_active: bool,
    created_at: DateTime,
    author: User
}

impl CommentWithAuthor{
    // CONSTRUCTORS
    pub fn new(
        article_id: Option<Uuid>,
        content: String,
        author: User,
    ) -> Self {
        let id = Uuid::new_v4();
        let created_at  = TimeHelper::now();
        let is_active = true;

        CommentWithAuthor {
            id,
            article_id,
            content,
            is_active,
            created_at,
            author
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        article_id: Option<Uuid>,
        content: String,
        is_active: bool,
        created_at: DateTime,
        author: User
    ) -> Self {
        CommentWithAuthor {
            id,
            article_id,
            content,
            is_active,
            created_at,
            author
        }
    }

    // GETTERS
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn article_id(&self) -> Option<Uuid> {
        self.article_id
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

    pub fn author(&self) -> &User {
        &self.author
    }
}
