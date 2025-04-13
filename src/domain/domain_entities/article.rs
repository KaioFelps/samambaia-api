use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use super::article_tag::ArticleTag;
use super::user::User;
use crate::domain::value_objects::changeset::{BlankChangeSet, ChangeSet, Changes};
use crate::domain::value_objects::slug::Slug;
use crate::libs::time::TimeHelper;
use crate::util::{verify_role_has_permission, RolePermissions};

#[derive(Clone, Debug, PartialEq)]
pub struct Article {
    id: Uuid,
    author_id: Uuid,
    cover_url: String,
    title: String,
    content: String,
    description: String,
    approved: bool,
    created_at: DateTime,
    updated_at: Option<DateTime>,
    slug: Slug,
    touched: bool,
    tags: ChangeSet<ArticleTag>,
}

impl Article {
    // CONSTRUCTORS
    pub fn new(
        author_id: Uuid,
        title: String,
        content: String,
        cover_url: String,
        description: String,
        tags: Vec<ArticleTag>,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_at = TimeHelper::now();
        let updated_at = None;

        let slug = Slug::new(id, title.clone());
        let tags_changeset = BlankChangeSet::default().into_filled(tags);

        Article {
            id,
            author_id,
            cover_url,
            title,
            content,
            approved: false,
            created_at,
            updated_at,
            slug,
            description,
            touched: false,
            tags: ChangeSet::Filled(tags_changeset),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_from_existing(
        id: Uuid,
        author_id: Uuid,
        cover_url: String,
        title: String,
        content: String,
        approved: bool,
        created_at: DateTime,
        updated_at: Option<DateTime>,
        slug: Slug,
        description: String,
        tags: Vec<ArticleTag>,
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
            slug,
            description,
            touched: false,
            tags: ChangeSet::new(tags),
        }
    }

    // METHODS

    fn touch(&mut self) {
        self.updated_at = Some(TimeHelper::now());
        self.touched = true;
    }

    pub fn disapprove_if_touched(&mut self, user: &User) {
        if !self.touched {
            return;
        }

        let user_can_modify_and_keep_approved =
            verify_role_has_permission(&user.role().unwrap(), RolePermissions::ApproveArticle);

        if !user_can_modify_and_keep_approved {
            self.approved = false;
        }
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

    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_approved(&mut self, approved: bool) {
        self.approved = approved;
    }

    pub fn get_tags(&self) -> Vec<&ArticleTag> {
        self.tags.get_current()
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
        self.title = title.clone();
        self.slug = Slug::new(self.id, title);

        self.touch();
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.touch();
    }

    pub fn set_tags(&mut self, tags: Vec<ArticleTag>) {
        let changeset = std::mem::take(&mut self.tags);

        self.tags = match changeset {
            ChangeSet::Blank(changeset) => changeset.into_filled(tags).into(),
            ChangeSet::Filled(changeset) => changeset.renew_with(tags).into(),
        };

        if self.tags.has_changes() {
            self.touch();
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.touch();
    }

    pub fn get_tags_changeset(&self) -> Option<Changes<'_, ArticleTag>> {
        self.tags.get_filled()?.differ().into()
    }

    pub fn flush_tags(&mut self) {
        let changeset = std::mem::take(&mut self.tags);
        self.tags = changeset.flush();
    }
}
