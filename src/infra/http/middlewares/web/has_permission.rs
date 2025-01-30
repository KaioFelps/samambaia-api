use std::future::{ready, Future, Ready};
use std::pin::Pin;

use crate::error::SamambaiaError;
use crate::util::RolePermissions;
use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, ResponseError};

use super::WebAuthUser;

#[derive(Clone)]
pub enum PermissionComparisonMode {
    All,
    Any,
}

/// # Has Permission Middleware
/// Checks if web auth user exists and has permission(s).
pub struct WebHasPermissionMiddleware {
    mode: PermissionComparisonMode,
    permissions: Vec<RolePermissions>,
}

impl WebHasPermissionMiddleware {
    pub fn new(permissions: Vec<RolePermissions>, mode: PermissionComparisonMode) -> Self {
        Self { mode, permissions }
    }
}

// S: 'static if working with async
impl<S, B> Transform<S, ServiceRequest> for WebHasPermissionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = WebHasPermissionService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(WebHasPermissionService {
            service,
            mode: self.mode.clone(),
            permissions: self.permissions.to_vec(),
        }))
    }
}

pub struct WebHasPermissionService<S> {
    service: S,
    mode: PermissionComparisonMode,
    permissions: Vec<RolePermissions>,
}

impl<S, B> Service<ServiceRequest> for WebHasPermissionService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_authorized =
            req.extensions()
                .get::<WebAuthUser>()
                .is_some_and(|user| match self.mode {
                    PermissionComparisonMode::All => self
                        .permissions
                        .iter()
                        .all(|required_permission| user.permissions.contains(required_permission)),

                    PermissionComparisonMode::Any => self
                        .permissions
                        .iter()
                        .any(|required_permission| user.permissions.contains(required_permission)),
                });

        if !is_authorized {
            let http_res = SamambaiaError::unauthorized_err()
                .error_response()
                .map_into_right_body();

            let (http_req, _) = req.into_parts();
            let res = ServiceResponse::new(http_req, http_res);

            return Box::pin(async move { Ok(res) });
        };

        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?.map_into_left_body()) })
    }
}
