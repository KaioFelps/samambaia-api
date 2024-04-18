use actix_web::web;

use crate::infra::http::middlewares::RequestUserMiddleware;
use crate::infra::http::controllers::users_controller::UsersController;

pub struct ApiRoutes;

impl ApiRoutes {
    pub fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
        web::scope("api")
            .wrap(RequestUserMiddleware)
            .configure(UsersController::register)
        );
    }
}
