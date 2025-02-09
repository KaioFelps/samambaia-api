use std::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, HttpMessage};
use serde::Serialize;

use crate::error::SamambaiaError;
use crate::infra::http::presenters::user::MappedUser;
use crate::util::RolePermissions;

pub mod has_permission;
pub mod web_auth_user;
pub mod web_request_user;

#[derive(Serialize, Clone, Debug)]
pub struct WebAuthUser {
    pub user: MappedUser,
    pub permissions: Vec<RolePermissions>,
}

#[derive(Clone, Debug)]
pub enum WebRequestUser {
    Ghast,
    User(WebAuthUser),
}

impl FromRequest for WebAuthUser {
    type Error = actix_web::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let user = req.extensions().get::<Self>().cloned();
        Box::pin(async move {
            match user {
                Some(user) => Ok(user),
                None => {
                    log::error!(
                        "Tried to extract a WebAuthUser that doesn't exist in the request."
                    );

                    Err(actix_web::Error::from(SamambaiaError::internal_err()))
                }
            }
        })
    }
}
