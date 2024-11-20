use actix_web::body::EitherBody;
use actix_web::dev::{self, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::middleware::Next;
use actix_web::HttpMessage;
use actix_web::{Error, ResponseError};
use actix_web_lab::__reexports::futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::error::DomainError;
use crate::infra::http::extractors::req_user::ReqUser;

/**
# Authentication Middleware
Apply this middleware to the routes that must be available only for logged-in users.
If applied and there is no ReqUser on the request object, it will return a 401 code response.

## Errors
- Will return 401 error if there is no ReqUser on the request Extension object.

## Usage
This middleware must be called from the `from_fn`.

```rs
// users_controller.rs
use actix_web_lab::middleware::from_fn;

pub struct UsersController {};

impl UsersController {
   pub fn register(cfg: &mut web::ServiceConfig) {
       cfg.service(web::scope("/users")
           // this route now is only available for logged in user
           // the route's method can access the payload
           .route("/new", web::post().to(Self::new).wrap(from_fn(authentication_middleware)))
       );
   }

   fn new(..., user: web::ReqData<ReqUser>) {
       // ...
   }
}
```
*/
pub async fn authentication_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<B>>, Error> {
    log::info!("Request going through Authentication Middleware.");
    let has_user = req.extensions().contains::<ReqUser>();

    if !has_user {
        log::info!("Request will be blocked by Authentication Middleware because there is no authenticated user.");

        let http_res = DomainError::unauthorized_err().error_response();
        let (http_req, _) = req.into_parts();
        let res = ServiceResponse::new(http_req, http_res);

        return Ok(res.map_into_right_body());
    }

    log::info!("Request passing successfully through Authentication Middleware.");

    next.call(req)
        .await
        .map(ServiceResponse::map_into_left_body)
}

/**
# Authentication Middleware
Apply this middleware to the routes that must be available only for logged-in users.
If applied and there is no ReqUser on the request object, it will return a 401 code response.

## Errors
- Will return 401 error if there is no ReqUser on the request Extension object.

## Usage
```rs
// users_controller.rs
pub struct UsersController {};

impl UsersController {
   pub fn register(cfg: &mut web::ServiceConfig) {
       cfg.service(web::scope("/users")
           // this route now is only available for logged in user
           // the route's method can access the payload
           .route("/new", web::post().to(Self::new).wrap(authentication_middleware))
       );
   }

   fn new(..., user: web::ReqData<ReqUser>) {
       // ...
   }
}
```
 */
pub struct AuthenticationMiddleware;
impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticationMiddlewareTransform<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareTransform { service }))
    }
}

pub struct AuthenticationMiddlewareTransform<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        log::info!("Request going through Authentication Middleware.");
        let has_user = request.extensions().contains::<ReqUser>();

        if !has_user {
            log::info!("Request will be blocked by Authentication Middleware because there is no authenticated user.");

            let http_res = DomainError::unauthorized_err()
                .error_response()
                .map_into_right_body();
            let (http_req, _) = request.into_parts();
            let res = ServiceResponse::new(http_req, http_res);

            return Box::pin(async { Ok(res) });
        }

        log::info!("Request passing successfully through Authentication Middleware.");

        let res = self.service.call(request);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
