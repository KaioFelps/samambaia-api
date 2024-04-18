// use actix_web::body::MessageBody;
use actix_web::body::BoxBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web::HttpResponse;
use actix_web_lab::middleware::Next;

use crate::errors::unauthorized_error::UnauthorizedError;
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
pub async fn authentication_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error>{
    log::info!("Request going through Authentication Middleware.");
    let has_user = req.extensions().contains::<ReqUser>();

    if !has_user {
        log::info!("Request will be blocked by Authentication Middleware because there is no authenticated user.");

        let http_res = HttpResponse::Unauthorized().json(UnauthorizedError::new());
        let (http_req, _) = req.into_parts();
        let res = ServiceResponse::new(http_req, http_res);

        return Ok(res)
    }

    log::info!("Request passing successfully through Authentication Middleware.");
    
    next.call(req).await
}
