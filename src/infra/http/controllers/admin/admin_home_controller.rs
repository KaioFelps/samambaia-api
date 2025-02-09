use actix_web::web::Data;
use actix_web::{web, HttpRequest};
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};

use crate::domain::factories::analytics::get_summary_service_factory;
use crate::error::IntoSamambaiaError;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::infra::sea::sea_service::SeaService;
use crate::util::RolePermissions;

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
    async fn home(req: HttpRequest, sea_service: Data<SeaService>) -> AppResponse {
        let summary_service = get_summary_service_factory::exec(&sea_service);

        let summary = summary_service.exec().await?;

        Inertia::render_with_props(
            &req,
            "admin/index".into(),
            hashmap!["summary" => InertiaProp::data(summary)],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }
}
