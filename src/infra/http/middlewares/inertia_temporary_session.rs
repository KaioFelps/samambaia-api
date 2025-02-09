use std::collections::HashMap;
use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use actix_web_lab::__reexports::futures_util::future::LocalBoxFuture;
use inertia_rust::actix::{is_inertia_response, SessionErrors};
use inertia_rust::{InertiaSessionToReflash, InertiaTemporarySession};
use log::error;
use serde_json::Map;

use crate::configs::app::SESSION_FLASH_KEY;

pub struct ReflashTemporarySessionMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ReflashTemporarySessionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ReflashTemporarySession<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ReflashTemporarySession { service }))
    }
}

pub struct ReflashTemporarySession<S> {
    service: S,
}

const ERRORS_KEY: &str = "_errors";
const PREV_REQ_KEY: &str = "_prev_req_url";
const CURR_REQ_KEY: &str = "_curr_req_url";

impl<S, B> Service<ServiceRequest> for ReflashTemporarySession<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();

        let errors = session.remove(ERRORS_KEY).map(|errors| {
            serde_json::from_str(&errors).unwrap_or_else(|err| {
                error!("Failed to serialize session errors: {}", err);
                Map::new()
            })
        });

        let before_prev_url = session
            .get::<String>(PREV_REQ_KEY)
            .unwrap_or(None)
            .unwrap_or("/".into());

        let prev_url = session
            .get::<String>(CURR_REQ_KEY)
            .unwrap_or(None)
            .unwrap_or("/".into());

        // ---

        let temporary_session = InertiaTemporarySession {
            errors: errors.clone(),
            prev_req_url: prev_url.clone(),
        };

        req.extensions_mut().insert(temporary_session);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            let req = res.request();
            let session = req.get_session();

            // If it's not a Inertia neither redirect response, it might be assets response
            // then, reflash everything
            let (prev_url, curr_url, optional_errors) = if !is_inertia_response(&res) {
                if let Some(flash_messages) = session.remove(SESSION_FLASH_KEY) {
                    if let Ok(flash_messages) =
                        serde_json::from_str::<HashMap<String, String>>(&flash_messages)
                    {
                        if let Err(err) = session.insert(SESSION_FLASH_KEY, flash_messages) {
                            error!("Failed to reflash flash messages: {}", err);
                        };
                    }
                }

                (before_prev_url, prev_url, errors)
            } else {
                let inertia_session = req.extensions_mut().remove::<InertiaSessionToReflash>();

                // if it needs to reflash a temporary flash session, then
                // replace data from inertia session middleware with the same as before,
                // so that the further request generates the same InertiaTemporarySession,
                // containing the exactly same errors, previous url, and current url.
                //
                // otherwise, gets the previous request's URI and stores the current one's as the next
                // request "previous", moving the navigation history
                if let Some(InertiaSessionToReflash(inertia_session)) = inertia_session {
                    (
                        before_prev_url,
                        inertia_session.prev_req_url,
                        inertia_session.errors,
                    )
                } else {
                    let errors = req
                        .extensions_mut()
                        .remove::<SessionErrors>()
                        .map(|SessionErrors(errors)| errors);

                    (prev_url, req.uri().to_string(), errors)
                }
            };

            if let Err(err) = session.insert(ERRORS_KEY, optional_errors.unwrap_or_default()) {
                error!("Failed to add errors to session: {}", err);
            }

            if let Err(err) = session.insert(PREV_REQ_KEY, prev_url) {
                error!("Failed to update session previous request URL: {}", err);
            };

            if let Err(err) = session.insert(CURR_REQ_KEY, curr_url) {
                error!("Failed to update session current request URL: {}", err);
            };

            Ok(res)
        })
    }
}
