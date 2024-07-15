use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Left, Right};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::domain::factories::create_comment_report_service_factory;
use crate::domain::services::create_comment_report_service::CreateCommentReportParams;
use crate::infra::http::dtos::create_comment_report::CreateCommentReportDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::comment_report::CommentReportPresenter;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::util::generate_error_response;
use super::controller::ControllerTrait;

pub struct CommentReportsController;

impl ControllerTrait for CommentReportsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/comment_reports")
            // CREATE
            .route("/{comment_id}/new", web::post().to(Self::create).wrap(from_fn(authentication_middleware)))

            // READ
            .route("/{id}/get", web::get().to(Self::get))
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update))

            // DELETE
            .route("/{id}/delete", web::delete().to(Self::delete))
        );
    }
}

impl CommentReportsController {
    async fn create(
        user: web::ReqData<ReqUser>,
        comment_id: web::Path<Uuid>,
        body: web::Json<CreateCommentReportDto>
    ) -> impl Responder {
        match body.validate() {
            Err(err) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(err.field_errors())),
            Ok(_) => (),
        };

        let body = body.into_inner();

        let service = match create_comment_report_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(CreateCommentReportParams {
            user_id: user.user_id,
            content: body.content,
            comment_id: comment_id.into_inner(),
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let comment_report = result.unwrap();
        let mapped_comment_report = CommentReportPresenter::to_http(comment_report);

        return HttpResponse::Created().json(json!({"data": mapped_comment_report}));
    }

    async fn get() -> impl Responder {
        return HttpResponse::Ok().finish();
    }

    async fn list() -> impl Responder {
        return HttpResponse::Ok().finish();
    }

    async fn update() -> impl Responder {
        return HttpResponse::NoContent().finish();
    }

    async fn delete() -> impl Responder {
        return HttpResponse::NoContent().finish();
    }
}
