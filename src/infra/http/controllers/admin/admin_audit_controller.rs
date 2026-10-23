use actix_web::web::{self, Data, Query};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};

use crate::error::IntoSamambaiaError;
use crate::infra::http::controllers::AppResponseRedirect;
use crate::infra::http::dtos::controllers::audit::AdminListAuditLogsDto;
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::queries::audit_logs::{AuditLogsQuery, AuditLogsQueryParams};
use crate::infra::sea::sea_service::SeaService;
use crate::util::RolePermissions;

const AUDIT_LOGS_PER_PAGE: usize = 20;

pub struct AdminAuditController;

impl AdminAuditController {
    pub fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/auditoria")
                .wrap(WebHasPermissionMiddleware::new(
                    vec![RolePermissions::ReadAuditLogs],
                    PermissionComparisonMode::Any,
                ))
                .route("", web::get().to(Self::manage_audit_logs)),
        );
    }

    async fn manage_audit_logs(
        req: HttpRequest,
        db_conn: Data<SeaService>,
        query: Query<AdminListAuditLogsDto>,
        auth: WebAuthUser,
    ) -> AppResponseRedirect {
        log::warn!("chegou aqui sim");
        let query = match query
            .into_inner()
            .validate_or_back(&req)
            .map_err(Either::Right)
        {
            Ok(query) => query,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let logs = AuditLogsQuery::new(&db_conn)
            .exec(AuditLogsQueryParams {
                staff_role: auth.user.role().unwrap(),
                page: query.page.unwrap_or(1),
                per_page: AUDIT_LOGS_PER_PAGE,
                search_query: query.search,
            })
            .await?;

        Inertia::render_with_props(
            &req,
            "admin/audit/index".into(),
            hashmap![
                "logs" => InertiaProp::data(logs)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
        .map(Either::Left)
    }
}
