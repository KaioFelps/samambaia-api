use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Left, Right};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::{
    create_comment_report_service_factory,
    delete_comment_report_service_factory,
    solve_comment_report_service_factory,
    fetch_many_comment_reports_service_factory
};
use crate::domain::services::create_comment_report_service::CreateCommentReportParams;
use crate::domain::services::delete_comment_report_service::DeleteCommentReportParams;
use crate::domain::services::fetch_many_comment_reports_service::{CommentReportServiceQuery, FetchManyCommentReportsParams};
use crate::domain::services::solve_comment_report_service::SolveCommentReportParams;
use crate::infra::http::dtos::create_comment_report::CreateCommentReportDto;
use crate::infra::http::dtos::list_comment_reports::ListCommentReportsDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::comment_report::{CommentReportPresenter, MappedCommentReport};
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::util::generate_error_response;
use super::controller::ControllerTrait;

pub struct CommentReportsController;

impl ControllerTrait for CommentReportsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/comment_reports")
            // REPORT A COMMENT
            .route("/{comment_id}/new", web::post().to(Self::create).wrap(from_fn(authentication_middleware)))

            // GET A PAGINATED LIST OF COMMENT REPORTS
            .route("/list", web::get().to(Self::list).wrap(from_fn(authentication_middleware)))
            
            // SOLVE A COMMENT REPORT
            .route("/{id}/solve", web::patch().to(Self::update).wrap(from_fn(authentication_middleware)))

            // DESTROY A COMMENT REPORT
            .route("/{id}/delete", web::delete().to(Self::delete).wrap(from_fn(authentication_middleware)))
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

    async fn list(query: web::Query<ListCommentReportsDto>) -> impl Responder {
        let service = match fetch_many_comment_reports_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let ListCommentReportsDto {
            per_page,
            page,
            solved,
            solved_by,
            content
        } = query.into_inner();

        let query = {
            if solved_by.is_some() {
                Some(CommentReportServiceQuery::SolvedBy(solved_by.unwrap()))
            } else if solved.is_some() {
                Some(CommentReportServiceQuery::Solved(solved.unwrap()))
            } else if content.is_some() {
                Some(CommentReportServiceQuery::Content(content.unwrap()))
            } else {
                None
            }
        };

        let result = service.exec(FetchManyCommentReportsParams {
            query,
            per_page: if per_page.is_some() { Some(per_page.unwrap() as u32) } else { None },
            page
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let comment_reports_paginated_data = result.unwrap();
        let mapped_reports = comment_reports_paginated_data.data.into_iter().map(CommentReportPresenter::to_http).collect::<Vec<MappedCommentReport>>();

        return HttpResponse::Ok().json(CommentReportPresenter::to_json_paginated_wrapper(
            mapped_reports,
            PaginationPresenter::to_http(comment_reports_paginated_data.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE))
        ));
    }

    async fn update(user: web::ReqData<ReqUser>, report_id: web::Path<i32>) -> impl Responder {
        let service = match solve_comment_report_service_factory::exec().await {
          Left(service) => service,
            Right(error) => return error,
        };

        let user = user.into_inner();

        let result = service.exec(SolveCommentReportParams {
            staff_role: user.user_role.unwrap(),
            com_report_id: report_id.into_inner(),
            staff_id: user.user_id
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }

    async fn delete(user: web::ReqData<ReqUser>, report_id: web::Path<i32>) -> impl Responder {
        let service = match delete_comment_report_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(DeleteCommentReportParams {
            com_report_id: report_id.into_inner(),
            staff_role: user.into_inner().user_role.unwrap()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }
}
