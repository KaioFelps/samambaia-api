use std::future::{ready, Future, Ready};
use std::pin::Pin;

use crate::error::SamambaiaError;
use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, ResponseError};

use super::WebRequestUser;

/**
# Web Authenticated User Middleware

Adds the request user to the extensions. If user is a ghast, it refuses the request.
```
*/
pub struct WebAuthUserMiddleware;

// S: 'static if working with async
impl<S, B> Transform<S, ServiceRequest> for WebAuthUserMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = WebAuthUserService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(WebAuthUserService { service }))
    }
}

pub struct WebAuthUserService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for WebAuthUserService<S>
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
        let auth_user = match req.extensions().get::<WebRequestUser>().cloned() {
            None => None,
            Some(user) => match user {
                WebRequestUser::Ghast => None,
                WebRequestUser::User(user) => Some(user),
            },
        };

        match auth_user {
            None => {
                let http_res = SamambaiaError::unauthorized_err()
                    .error_response()
                    .map_into_right_body();

                let (http_req, _) = req.into_parts();
                let res = ServiceResponse::new(http_req, http_res);

                Box::pin(async move { Ok(res) })
            }

            Some(user) => {
                req.extensions_mut().insert(user);
                let fut = self.service.call(req);
                Box::pin(async move { Ok(fut.await?.map_into_left_body()) })
            }
        }
    }
}
