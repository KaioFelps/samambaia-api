use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::libs::time::TimeHelper;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamRole {
    id: Uuid,
    title: String,
    description: String,
    created_at: DateTime,
}

impl TeamRole {
    // CONSTRUCTORS
    pub fn new(title: String, description: String) -> Self {
        let id = Uuid::new_v4();
        let created_at = TimeHelper::now();

        TeamRole {
            id,
            title,
            description,
            created_at,
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        title: String,
        description: String,
        created_at: DateTime,
    ) -> Self {
        TeamRole {
            id,
            title,
            description,
            created_at,
        }
    }

    // GETTERS
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    // SETTERS
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
