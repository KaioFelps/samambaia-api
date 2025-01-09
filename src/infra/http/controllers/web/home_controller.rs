use actix_web::{web, HttpRequest};
use inertia_rust::{Inertia, InertiaFacade};

use crate::{
    error::IntoDomainError,
    infra::http::controllers::{controller::ControllerTrait, AppResponse},
};

pub struct HomeController;

impl ControllerTrait for HomeController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.route("/", web::get().to(Self::home));
    }
}

impl HomeController {
    async fn home(req: HttpRequest) -> AppResponse {
        Inertia::render(&req, "index".into())
            .await
            .map_err(IntoDomainError::into_domain_error)
    }
}
