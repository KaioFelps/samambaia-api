use actix_web::web::{self, Data};

use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::tools::imager_controller::ImagerController;
use crate::infra::http::routes::route::RouteTrait;
use crate::infra::imager::factory::ImagerFactory;

pub struct ToolsRouter;

impl RouteTrait for ToolsRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        let imager = match ImagerFactory::get_imager() {
            Err(err) => panic!("Could not instantiate an imager: {err}"),
            Ok(imager) => Data::new(imager),
        };

        cfg.service(web::scope("tools").configure(ImagerController::register))
            .app_data(imager);
    }
}
