use actix_web::web;

use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::tools::imager_controller::ImagerController;
use crate::infra::http::routes::route::RouteTrait;

pub struct ToolsRouter;

impl RouteTrait for ToolsRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(web::scope("tools").configure(ImagerController::register));
    }
}
