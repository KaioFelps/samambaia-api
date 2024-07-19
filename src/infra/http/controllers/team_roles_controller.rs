use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Left, Right};
use uuid::Uuid;
use validator::Validate;
use crate::core::pagination::DEFAULT_PER_PAGE;
use super::controller::ControllerTrait;
use crate::domain::factories::{
    create_team_role_service_factory,
    delete_team_role_service_factory,
    update_team_role_service_factory,
    fetch_many_team_roles_service_factory
};
use crate::domain::repositories::team_role_repository::TeamRoleQueryType;
use crate::domain::services::create_team_role_service::CreateTeamRoleParams;
use crate::domain::services::delete_team_role_service::DeleteTeamRoleParams;
use crate::domain::services::fetch_many_team_roles_service::FetchManyTeamRolesParams;
use crate::domain::services::update_team_role_service::UpdateTeamRoleParams;
use crate::infra::http::dtos::create_team_role::CreateTeamRoleDto;
use crate::infra::http::dtos::list_team_role::ListTeamRoleDto;
use crate::infra::http::dtos::update_team_role::UpdateTeamRoleDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::team_role::{MappedTeamRole, TeamRolePresenter};
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::util::generate_error_response;

pub struct TeamRolesController;

impl ControllerTrait for TeamRolesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/team_roles")
            // CREATE
            .route("/new", web::post().to(Self::create).wrap(from_fn(authentication_middleware)))

            // READ
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update))

            // DELETE
            .route("/{id}/delete", web::delete().to(Self::delete))
        );
    }
}

impl TeamRolesController {
    async fn create(user: web::ReqData<ReqUser>, body: web::Json<CreateTeamRoleDto>) -> impl Responder {
        match body.validate() {
            Err(e) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(e.field_errors())),
            Ok(_) => ()
        };

        let body = body.into_inner();

        let service = match create_team_role_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(CreateTeamRoleParams {
            staff_role: user.into_inner().user_role.unwrap(),
            title: body.title,
            description: body.description
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let team_role = result.unwrap();
        let mapped_team_role = TeamRolePresenter::to_http(team_role);

        return HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_team_role
        });
    }

    async fn list(query: web::Query<ListTeamRoleDto>) -> impl Responder {
        let service = match fetch_many_team_roles_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        match query.validate() {
            Err(e) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(e.field_errors())),
            Ok(_) => ()
        }

        let query = query.into_inner();

        let result = service.exec(FetchManyTeamRolesParams {
            per_page: if query.per_page.is_some() { Some(query.per_page.unwrap() as u32) } else { None },
            page: query.page,
            query: if query.title.is_some() { Some(TeamRoleQueryType::Title(query.title.unwrap())) } else { None },
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let result = result.unwrap();
        let mapped_team_roles = result.data.into_iter().map(TeamRolePresenter::to_http).collect::<Vec<MappedTeamRole>>();
        let mapped_pagination = PaginationPresenter::to_http(result.pagination, query.per_page.unwrap_or(DEFAULT_PER_PAGE));

        return HttpResponse::Ok().json(TeamRolePresenter::to_json_paginated_wrapper(mapped_team_roles, mapped_pagination));
    }

    async fn update(
        user: web::ReqData<ReqUser>,
        team_role_id: web::Path<Uuid>,
        body: web::Json<UpdateTeamRoleDto>
    ) -> impl Responder {
        match body.validate() {
            Err(e) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(e.field_errors())),
            Ok(_) => (),
        };

        let service = match update_team_role_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let user = user.into_inner();
        let UpdateTeamRoleDto {title, description} = body.into_inner();

        let result = service.exec(UpdateTeamRoleParams {
            title,
            staff_role: user.user_role.unwrap(),
            description,
            team_role_id: team_role_id.into_inner()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let team_role = result.unwrap();
        let mapped_team_role = TeamRolePresenter::to_http(team_role);

        return HttpResponse::Ok().json(JsonWrappedEntity {
            data: mapped_team_role
        });
    }

    async fn delete(user: web::ReqData<ReqUser>, team_role_id: web::Path<Uuid>) -> impl Responder {
        let service = match delete_team_role_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(DeleteTeamRoleParams {
            team_role_id: team_role_id.into_inner(),
            staff_role: user.into_inner().user_role.unwrap()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }
}
