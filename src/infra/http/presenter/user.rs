use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::domain::domain_entities::{role::Role, user::User};

#[derive(Serialize, Deserialize)]
pub struct MappedUser {
    id: Uuid,
    nickname: String,
    created_at: DateTime,
    role: Role,
}

pub struct UserPresenter;

impl UserPresenter {
    pub fn to_http(user: User) -> MappedUser {
        MappedUser {
            nickname: user.nickname().into(),
            id: user.id(),
            role: user.role().unwrap(),
            created_at: user.created_at()
        }
    }
}
