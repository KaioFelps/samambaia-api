use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::domain::domain_entities::{role::Role, user::User};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MappedUser {
    id: Uuid,
    nickname: String,
    createdAt: DateTime,
    role: Role,
}

pub struct UserPresenter;

impl UserPresenter {
    pub fn to_http(user: User) -> MappedUser {
        MappedUser {
            nickname: user.nickname().into(),
            id: user.id(),
            role: user.role().unwrap(),
            createdAt: user.created_at()
        }
    }
}
