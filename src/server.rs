use crate::infra::http::routes::web::WebRoutes;
use crate::infra::http::routes::{api::ApiRoutes, route::RouteTrait};
use crate::infra::sea::sea_service::SeaService;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{middleware, web, App, HttpResponse};
use serde_json::json;

pub struct ServerFactory;

impl ServerFactory {
    pub fn exec() -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .configure(ApiRoutes::register)
            .configure(WebRoutes::register)
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(json!({"error": err.to_string()})), // or
                                                                  // .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
    }

    pub fn exec_with_sea(
        sea_service: web::Data<SeaService>,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        Self::exec().app_data(sea_service)
    }
}
