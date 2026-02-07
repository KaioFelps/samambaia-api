use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::{App, HttpResponse, middleware, web};
use serde_json::json;

use crate::infra::http::routes::api::ApiRoutes;
use crate::infra::http::routes::route::RouteTrait;
use crate::infra::http::routes::tools::ToolsRouter;
use crate::infra::http::routes::web::WebRoutes;
use crate::infra::sea::sea_service::SeaService;

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
            .wrap(
                middleware::DefaultHeaders::new()
                    .add((header::SERVER, "actix"))
                    .add(("x-powered-by", "rust")),
            )
            .configure(ApiRoutes::register)
            .configure(ToolsRouter::register)
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
