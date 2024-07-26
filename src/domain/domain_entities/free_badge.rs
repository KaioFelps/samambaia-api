use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::libs::time::TimeHelper;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreeBadge {
    id: Uuid,
    image: String,
    code: String,
    link: String,
    link_is_external: bool,
    created_at: NaiveDateTime,
    available_until: Option<NaiveDateTime>
}

impl FreeBadge {
    pub fn new(
        code: String,
        image: String,
        link: String,
        link_is_external: bool,
        available_until: Option<NaiveDateTime>
    ) -> FreeBadge {
        let id = Uuid::new_v4();
        let created_at  = TimeHelper::now();

        FreeBadge {
            id,
            code,
            image,
            link,
            link_is_external,
            available_until,
            created_at
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        code: String,
        image: String,
        link: String,
        link_is_external: bool,
        created_at: NaiveDateTime,
        available_until: Option<NaiveDateTime>
    ) -> FreeBadge {
        FreeBadge {
            id,
            code,
            image,
            link,
            link_is_external,
            available_until,
            created_at
        }
    }


    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn image(&self) -> &String {
        &self.image
    }

    pub fn link(&self) -> &String {
        &self.link
    }

    pub fn link_is_external(&self) -> bool {
        self.link_is_external
    }

    pub fn available_until(&self) -> Option<NaiveDateTime> {
        self.available_until
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}