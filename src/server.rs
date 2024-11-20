use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware, web, App, HttpResponse,
};
use serde_json::json;

use crate::infra::http::routes::api::ApiRoutes;
use crate::infra::http::routes::route::RouteTrait;

pub struct ServerFactory;

impl ServerFactory {
    pub fn exec_and_overwrite_db_url(
        db: String,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        std::env::set_var("DATABASE_URL", db);

        Self::exec()
    }

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
}
