use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

use crate::libs::time::TimeHelper;

pub struct CouncilAlertDraft {
    pinned: bool,
    title: String,
    content: String,
    created_at: NaiveDateTime,
}

#[derive(Clone, PartialEq, Eq, FromRow)]
pub struct CouncilAlert {
    id: i32,
    pinned: bool,
    title: String,
    content: String,
    created_at: NaiveDateTime,
}

impl CouncilAlertDraft {
    #[inline]
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            pinned: false,
            created_at: TimeHelper::now(),
        }
    }

    pub fn into_council_alert(self, id: i32) -> CouncilAlert {
        CouncilAlert::new_from_existing(id, self.title, self.content, self.pinned, self.created_at)
    }

    pub fn pin(&mut self) {
        self.pinned = true;
    }

    pub fn unpin(&mut self) {
        self.pinned = false;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}

impl CouncilAlert {
    pub fn new_draft(title: String, content: String) -> CouncilAlertDraft {
        CouncilAlertDraft::new(title, content)
    }

    pub fn new_from_existing(
        id: i32,
        title: String,
        content: String,
        pinned: bool,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            title,
            content,
            pinned,
            created_at,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn pin(&mut self) {
        self.pinned = true;
    }

    pub fn unpin(&mut self) {
        self.pinned = false;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}
