use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Left, Right};
use uuid::Uuid;
use validator::Validate;
use crate::core::pagination::DEFAULT_PER_PAGE;
use super::controller::ControllerTrait;
use crate::domain::factories::{
    create_team_user_service_factory,
    delete_team_user_service_factory,
    update_team_user_service_factory,
    fetch_many_team_users_service_factory
};
use crate::domain::repositories::team_user_repository::TeamUserQueryType;
use crate::domain::services::create_team_user_service::CreateTeamUserParams;
use crate::domain::services::delete_team_user_service::DeleteTeamUserParams;
use crate::domain::services::fetch_many_team_users_service::FetchManyTeamUsersParams;
use crate::domain::services::update_team_user_service::UpdateTeamUserParams;
use crate::infra::http::dtos::create_team_user::CreateTeamUserDto;
use crate::infra::http::dtos::list_team_user::ListTeamUsersDto;
use crate::infra::http::dtos::update_team_user::UpdateTeamUserDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::team_user::{MappedTeamUser, TeamUserPresenter};
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::util::generate_error_response;

pub struct TeamUsersController;

impl ControllerTrait for TeamUsersController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/team_users")
            // CREATE
            .route("/new", web::post().to(Self::create).wrap(from_fn(authentication_middleware)))

            // READ
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update).wrap(from_fn(authentication_middleware)))

            // DELETE
            .route("/{id}/delete", web::delete().to(Self::delete).wrap(from_fn(authentication_middleware)))
        );
    }
}

impl TeamUsersController {
    async fn create(
        body: web::Json<CreateTeamUserDto>,
        user: web::ReqData<ReqUser>,
    ) -> impl Responder {
        match body.validate() {
            Err(e) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(e.field_errors())),
            Ok(()) => (),
        };

        let service = match create_team_user_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let body = body.into_inner();

        let result = service.exec(CreateTeamUserParams {
            nickname: body.nickname,
            team_role_id: body.team_role_id,
            staff_role: user.into_inner().user_role.unwrap(),
            discord: body.discord,
            twitter: body.twitter,
            user_function: body.user_function
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let team_user = result.unwrap();
        let mapped_team_user = TeamUserPresenter::to_http(team_user);

        return HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_team_user
        });
    }

    async fn list(query: web::Query<ListTeamUsersDto>) -> impl Responder {
        let service = match fetch_many_team_users_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let ListTeamUsersDto {
            page,
            per_page,
            team_role_id,
            nickname
        } = query.into_inner();

        let query = {
            if team_role_id.is_some() {
                Some(TeamUserQueryType::TeamRole(team_role_id.unwrap()))
            } else if nickname.is_some() {
                Some(TeamUserQueryType::Nickname(nickname.unwrap()))
            } else {
                None
            }
        };

        let result = service.exec(FetchManyTeamUsersParams {
            per_page: if per_page.is_some() { Some(per_page.unwrap() as u32) } else { None },
            query,
            page
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let result = result.unwrap();
        let mapped_team_users = result.data.into_iter().map(TeamUserPresenter::to_http).collect::<Vec<MappedTeamUser>>();
        let mapped_pagination = PaginationPresenter::to_http(result.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE));

        return HttpResponse::Ok().json(TeamUserPresenter::to_json_paginated_wrapper(mapped_team_users, mapped_pagination));
    }

    async fn update(
        body: web::Json<UpdateTeamUserDto>,
        user: web::ReqData<ReqUser>,
        team_user_id: web::Path<Uuid>
    ) -> impl Responder {
        match body.validate() {
            Err(e) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(e.field_errors())),
            Ok(()) => (),
        };

        let service= match update_team_user_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let body = body.into_inner();

        let result = service.exec(UpdateTeamUserParams {
            nickname: body.nickname,
            team_role_id: body.team_role_id,
            twitter: body.twitter,
            discord: body.discord,
            user_function: body.user_function,
            staff_role: user.into_inner().user_role.unwrap(),
            team_user_id: team_user_id.into_inner()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let team_user = result.unwrap();
        let mapped_team_user = TeamUserPresenter::to_http(team_user);

        return HttpResponse::Ok().json(JsonWrappedEntity {
            data: mapped_team_user
        });
    }

    async fn delete(user: web::ReqData<ReqUser>, team_user_id: web::Path<Uuid>) -> impl Responder {
        let service = match delete_team_user_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(DeleteTeamUserParams {
            staff_role: user.into_inner().user_role.unwrap(),
            team_user_id: team_user_id.into_inner()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }
}
