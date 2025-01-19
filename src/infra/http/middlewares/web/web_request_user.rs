use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;

use crate::domain::factories::identity::get_user_service_factory;
use crate::domain::services::identity::get_user_service::GetUserServiceParams;
use crate::error::SamambaiaError;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::http::presenters::user::UserPresenter;
use crate::infra::sea::sea_service::SeaService;
use crate::util::RolePermissions;
use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error, HttpMessage, ResponseError};
use uuid::Uuid;

use super::{WebAuthUser, WebRequestUser};

/**
# Web Request User Middleware

Extracts user from sessions. If the request user cannot be found, it's considered a ghast.

## Errors
The middleware return error if it fails to connect to the database.
```
*/
pub struct WebRequestUserMiddleware;

// S: 'static if working with async
impl<S, B> Transform<S, ServiceRequest> for WebRequestUserMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = WebRequestUserService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(WebRequestUserService {
            service: Rc::new(service),
        }))
    }
}

pub struct WebRequestUserService<S> {
    // service: Rc<S>
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for WebRequestUserService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let user = get_session_user(&mut req).await;
            match user {
                Err(err) => {
                    let http_res = err.error_response().map_into_right_body();
                    let (http_req, _) = req.into_parts();
                    let res = ServiceResponse::new(http_req, http_res);

                    Ok(res)
                }

                Ok(user) => {
                    req.extensions_mut().insert(user);
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
            }
        })
    }
}

async fn get_session_user(req: &mut ServiceRequest) -> Result<WebRequestUser, SamambaiaError> {
    let db_conn = req
        .extract::<web::Data<SeaService>>()
        .await
        .map_err(|err| {
            log::error!(
                "Failed to extract SeaService from request in WebAuthUserMiddleware: {}",
                err
            );
            SamambaiaError::internal_err()
        })?;

    let find_user = get_user_service_factory::exec(&db_conn);

    let user_id = match match req
        .get_session()
        .get::<String>("__user_id__")
        .unwrap_or(None)
    {
        None => return Ok(WebRequestUser::Ghast),
        Some(id) => id,
    }
    .parse::<Uuid>()
    {
        Err(_) => return Ok(WebRequestUser::Ghast),
        Ok(id) => id,
    };

    let user = match match find_user.exec(GetUserServiceParams { user_id }).await {
        Err(_) => return Ok(WebRequestUser::Ghast),
        Ok(user) => user,
    } {
        None => return Ok(WebRequestUser::Ghast),
        Some(user) => user,
    };

    let permissions = RolePermissions::get_from_role(&user.role().unwrap());
    let user = UserPresenter::to_http(user);

    Ok(WebRequestUser::User(WebAuthUser { user, permissions }))
}
