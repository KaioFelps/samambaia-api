use actix_web::web;

use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::sessions_controller::SessionsController;
use crate::infra::http::middlewares::RequestUserMiddleware;
use crate::infra::http::controllers::users_controller::UsersController;
use crate::infra::http::routes::route::RouteTrait;

pub struct ApiRoutes;

impl RouteTrait for ApiRoutes {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
        web::scope("api")
            .wrap(RequestUserMiddleware)
            .configure(UsersController::register)
            .configure(SessionsController::register)
        );
    }
}
