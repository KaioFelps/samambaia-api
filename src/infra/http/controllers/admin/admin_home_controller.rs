use crate::error::IntoSamambaiaError;
use crate::infra::http::controllers::{controller::ControllerTrait, AppResponse};
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode, WebHasPermissionMiddleware,
};
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::util::RolePermissions;
use actix_web::{web, HttpRequest};
use inertia_rust::{Inertia, InertiaFacade};

pub struct AdminHomeController;

impl ControllerTrait for AdminHomeController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/gremio")
                .wrap(WebHasPermissionMiddleware::new(
                    vec![RolePermissions::AccessDashboard],
                    PermissionComparisonMode::All,
                ))
                .wrap(WebAuthUserMiddleware)
                .route("", web::get().to(Self::home)),
        );
    }
}

impl AdminHomeController {
    async fn home(req: HttpRequest) -> AppResponse {
        Inertia::render(&req, "Admin/Index".into())
            .await
            .map_err(IntoSamambaiaError::into_samambaia_error)
    }
}
