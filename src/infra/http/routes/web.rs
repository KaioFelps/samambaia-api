use std::path::Path;

use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::web::home_controller::HomeController;
use crate::infra::http::middlewares::RequestUserMiddleware;
use actix_web::web;
use inertia_rust::actix::InertiaMiddleware;

use super::route::RouteTrait;

pub struct WebRoutes;

impl RouteTrait for WebRoutes {
    fn register(cfg: &mut web::ServiceConfig) {
        // adds this service only if the directory exists
        if Path::new("public/bundle/assets").exists() {
            // serves vite assets from /assets path
            cfg.service(
                actix_files::Files::new("/assets", "./public/bundle/assets").prefer_utf8(true),
            );
        }

        cfg.service(
            web::scope("")
                .wrap(RequestUserMiddleware)
                .wrap(InertiaMiddleware::new())
                .configure(HomeController::register)
                .configure(|cfg| {
                    // serves public assets directly from /path
                    // needs to be the last service because it's a wildcard
                    cfg.service(actix_files::Files::new("/", "./public/").prefer_utf8(true));
                }),
        );
    }
}
