use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::libs::time::TimeHelper;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Announcement {
    id: Uuid,
    url: String,
    image: String,
    external: bool,
    description: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    author_id: Uuid,
}

impl Announcement {
    pub fn new(
        url: String,
        image: String,
        external: bool,
        author_id: Uuid,
        description: String,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_at = TimeHelper::now();
        let updated_at = None;

        Announcement {
            id,
            author_id,
            created_at,
            updated_at,
            description,
            external,
            image,
            url,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_from_existing(
        id: Uuid,
        url: String,
        image: String,
        external: bool,
        description: String,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
        author_id: Uuid,
    ) -> Self {
        Self {
            id,
            author_id,
            created_at,
            updated_at,
            description,
            external,
            image,
            url,
        }
    }

    // getters

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn image(&self) -> &String {
        &self.image
    }

    pub fn external(&self) -> &bool {
        &self.external
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<NaiveDateTime> {
        &self.updated_at
    }

    pub fn author_id(&self) -> &Uuid {
        &self.author_id
    }

    // setters

    pub fn set_url(&mut self, url: String) {
        self.url = url;
        self.touch();
    }

    pub fn set_image(&mut self, image: String) {
        self.image = image;
        self.touch();
    }

    pub fn set_external(&mut self, external: bool) {
        self.external = external;
        self.touch();
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.touch();
    }

    // helpers
    fn touch(&mut self) {
        self.updated_at = Some(TimeHelper::now());
    }
}
