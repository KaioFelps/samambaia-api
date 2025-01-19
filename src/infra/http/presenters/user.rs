use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::domain_entities::{role::Role, user::User};
use crate::infra::http::presenters::presenter::PresenterTrait;
#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct MappedUser {
    id: Uuid,
    nickname: String,
    createdAt: DateTime,
    role: Role,
}

pub struct UserPresenter;

impl PresenterTrait<User, MappedUser> for UserPresenter {
    fn to_http(user: User) -> MappedUser {
        MappedUser {
            nickname: user.nickname().into(),
            id: user.id(),
            role: user.role().unwrap(),
            createdAt: user.created_at(),
        }
    }
}
