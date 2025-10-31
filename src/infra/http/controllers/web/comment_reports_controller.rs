use actix_session::Session;
use actix_web::HttpRequest;
use actix_web::web::{self, Data, Json, Path, Redirect};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade};
use uuid::Uuid;

use crate::domain::domain_entities::comment_report::CommentReportTrait;
use crate::domain::factories::security::create_comment_report_service_factory;
use crate::domain::services::security::create_comment_report_service::CreateCommentReportParams;
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::dtos::create_comment_report::CreateCommentReportDto;
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::routes::route::RouteTrait;
use crate::infra::sea::sea_service::SeaService;

pub struct CommentReportsController;

impl RouteTrait for CommentReportsController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("comment_reports")
                .wrap(WebAuthUserMiddleware)
                .route("{comment_id}/new", web::post().to(Self::report_comment)),
        );
    }
}

impl CommentReportsController {
    pub async fn report_comment(
        req: HttpRequest,
        auth_user: WebAuthUser,
        comment_id: Path<Uuid>,
        body: Json<CreateCommentReportDto>,
        db_conn: Data<SeaService>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let service = create_comment_report_service_factory::exec(&db_conn);

        let report = service
            .exec(CreateCommentReportParams {
                comment_id: comment_id.into_inner(),
                content: body.content.unwrap(),
                user_id: auth_user.user.id(),
            })
            .await?;

        Session::flash_silently(
            &req,
            "commentReportSuccess",
            format!("Denúncia realizada: {}", report.message()),
        );

        Ok(Inertia::back(&req))
    }
}
