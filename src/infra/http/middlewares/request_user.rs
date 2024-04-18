use std::future::{ready, Future, Ready};
use std::pin::Pin;

use crate::infra::http::extractors::req_user::ReqUser;
use crate::{ENV_VARS, LOG_SEP, R_EOL};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use jsonwebtoken::DecodingKey;
use log::error;

use crate::infra::jwt::jwt_service::JwtService;

/**
 # Request User Middleware
 This middleware can be applied on any routes. It will try to get and extract User data from Authorization header.
 If there is no token, the ReqUser will be None. If it gets to extract the data, the user payload will be accessible from an extracto.

 ## Errors
 The middleware will return no errors.

 ## Usage
 ```rs
 // users_controller.rs
 pub struct UsersController {};

 impl UsersController {
    pub fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/users")
            // this route now is only available for logged in user
            // the route's method can access the payload
            .route("/new", web::post().to(Self::new).wrap(RequestUserMiddleware))
        );
    }

    fn new(..., user: Option<web::ReqData<ReqUser>>) {
        let user: ReqUser = token.unwrap().into_inner();
        // ...
    }
 }
 ```
 */
pub struct RequestUserMiddleware;

// S: 'static if working with async
impl<S, B> Transform<S, ServiceRequest> for RequestUserMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestUserService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // ready(Ok(RequestUserService { service: Rc::new(service) }))
        ready(Ok(RequestUserService { service }))
    }
}

pub struct RequestUserService<S> {
    // service: Rc<S>
    service: S
}

impl<S, B> Service<ServiceRequest> for RequestUserService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> /* + 'static */,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // let svc = self.service.clone();
        // Box::pin(async move { ... async code }

        log::info!("Request going through Request User Middleware.");

        let auth_header = req.headers().get("Authorization");
        let auth_header = {
            if let Some(raw_header) = auth_header {
                raw_header.to_str().ok()
            } else {
                None
            }
        };

        let auth_token = {
            if let None = auth_header {
                None
            }
            else if !auth_header.unwrap().to_string().starts_with("Bearer ") {
                None
            }
            else {
                let token = auth_header.unwrap().to_string();
                let token = token.replace("Bearer", "");
                let token = token.trim().to_string();
                Some(token)
            }
        };

        if let Some(token) = auth_token {
            let jwt_service = JwtService {};
            
            let jwt_t = jwt_service.decode_jwt(
                token,
                DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref())
            );

            match jwt_t {
                Err(e) => {
                    error!(
                        "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Request User Middleware, decoding the token: {R_EOL}{e}{R_EOL}{LOG_SEP}{R_EOL}",
                    );
                },
                Ok(user) => {
                    req.extensions_mut().insert::<ReqUser>(user);
                }
            };
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}
