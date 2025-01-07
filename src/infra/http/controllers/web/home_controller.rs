use actix_web::{web, HttpRequest, Responder};
use inertia_rust::{Inertia, InertiaFacade};

use crate::infra::http::controllers::controller::ControllerTrait;

pub struct HomeController;

impl ControllerTrait for HomeController {
    fn register(cfg: &mut web::ServiceConfig) {}
}

impl HomeController {}
