use crate::libs::time::TimeHelper;
use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use super::role::Role;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    id: Uuid,
    nickname: String,
    password: String,
    created_at: DateTime,
    last_login: Option<DateTime>,
    role: Option<Role>,
}

impl User {
    // CONSTRUCTOR
    pub fn new(nickname: String, password: String, role: Option<Role>) -> Self {
        let id = Uuid::new_v4();

        let created_at = TimeHelper::now();
        let last_login = None;

        User {
            id,
            nickname,
            password,
            created_at,
            last_login,
            role,
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        nickname: String,
        password: String,
        created_at: DateTime,
        last_login: Option<DateTime>,
        role: Option<Role>,
    ) -> Self {
        User {
            id,
            nickname,
            password,
            created_at,
            last_login,
            role,
        }
    }

    // GETTERS
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn nickname(&self) -> &str {
        self.nickname.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub fn last_login(&self) -> Option<DateTime> {
        self.last_login
    }

    pub fn role(&self) -> Option<Role> {
        self.role.clone()
    }

    // SETTERS

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }

    pub fn set_last_login(&mut self, last_login: Option<DateTime>) {
        self.last_login = last_login;
    }

    pub fn set_role(&mut self, role: Option<Role>) {
        self.role = role;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
