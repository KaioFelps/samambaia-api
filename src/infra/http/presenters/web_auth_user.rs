use serde::Serialize;

use super::presenter::PresenterTrait;
use super::user::{MappedUser, UserPresenter};
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::util::RolePermissions;

#[derive(Serialize)]
pub struct MappedWebAuthUser {
    user: MappedUser,
    permissions: Vec<RolePermissions>,
}

pub struct WebAuthUserPresenter;

impl PresenterTrait<WebAuthUser, MappedWebAuthUser> for WebAuthUserPresenter {
    fn to_http(entity: WebAuthUser) -> MappedWebAuthUser {
        MappedWebAuthUser {
            user: UserPresenter::to_http(entity.user),
            permissions: entity.permissions,
        }
    }
}
