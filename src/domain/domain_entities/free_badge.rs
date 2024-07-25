use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::libs::time::TimeHelper;

#[derive(Debug)]
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

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_image(&self) -> &String {
        &self.image
    }

    pub fn get_link(&self) -> &String {
        &self.link
    }

    pub fn get_link_is_external(&self) -> bool {
        self.link_is_external
    }

    pub fn get_available_until(&self) -> Option<NaiveDateTime> {
        self.available_until
    }

    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}