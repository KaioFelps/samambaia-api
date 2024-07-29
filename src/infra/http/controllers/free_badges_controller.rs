use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Left, Right};
use uuid::Uuid;
use validator::Validate;
use crate::core::pagination::DEFAULT_PER_PAGE;
use super::controller::ControllerTrait;
use crate::domain::factories::{
    create_free_badge_service_factory,
    fetch_many_free_badges_service_factory,
    update_free_badge_service_factory,
    delete_free_badge_service_factory
};
use crate::domain::services::create_free_badge_service::CreateFreeBadgeParams;
use crate::domain::services::delete_free_badge_service::DeleteFreeBadgeParams;
use crate::domain::services::fetch_many_free_badges_service::FetchManyFreeBadgesParams;
use crate::domain::services::update_free_badge_service::UpdateFreeBadgeParams;
use crate::infra::http::dtos::create_free_badge::CreateFreeBadgeDto;
use crate::infra::http::dtos::simple_pagination_query::SimplePaginationQueryDto;
use crate::infra::http::dtos::update_free_badge::UpdateFreeBadgeDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::free_badge::{FreeBadgePresenter, MappedFreeBadge};
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::util::generate_error_response;

pub struct FreeBadgesController;

impl ControllerTrait for FreeBadgesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/free_badges")
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

impl FreeBadgesController {
    async fn create(user: web::ReqData<ReqUser>, body: web::Json<CreateFreeBadgeDto>) -> impl Responder {
        match body.validate() {
            Ok(()) => (),
            Err(error) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(error.field_errors()))
        };

        let body = body.into_inner();

        let service = match create_free_badge_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let free_badge = service.exec(CreateFreeBadgeParams {
            user_role: user.into_inner().user_role.unwrap(),
            code: body.code,
            image: body.image,
            link: body.link,
            link_is_external: body.link_is_external,
            available_until: body.available_until,
        }).await;

        if free_badge.is_err() {
            return generate_error_response(free_badge.unwrap_err());
        }

        let free_badge = free_badge.unwrap();

        return HttpResponse::Created().json(JsonWrappedEntity {
            data: FreeBadgePresenter::to_http(free_badge)
        });
    }

    async fn list(query: web::Query<SimplePaginationQueryDto>) -> impl Responder {
        let service = match fetch_many_free_badges_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

        let result = service.exec(FetchManyFreeBadgesParams {
            page: query.page,
            per_page: if query.per_page.is_some() { Some(query.page.unwrap() as u32) } else { None }
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let service_response = result.unwrap();

        let mapped_free_badges = service_response.data.into_iter().map(FreeBadgePresenter::to_http).collect::<Vec<MappedFreeBadge>>();
        let mapped_pagination = PaginationPresenter::to_http(service_response.pagination, query.per_page.unwrap_or(DEFAULT_PER_PAGE));

        return HttpResponse::Ok().json(FreeBadgePresenter::to_json_paginated_wrapper(mapped_free_badges, mapped_pagination));
    }

    async fn update(
        user: web::ReqData<ReqUser>,
        body: web::Json<UpdateFreeBadgeDto>,
        free_badge_id: web::Path<Uuid>
    ) -> impl Responder {
        match body.validate() {
            Err(error) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(error.field_errors())),
            _ => (),
        }

        let body = body.into_inner();

        let service = match update_free_badge_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(UpdateFreeBadgeParams {
            user_role: user.into_inner().user_role.unwrap(),
            code: body.code,
            image: body.image,
            link: body.link,
            link_is_external: body.link_is_external,
            available_until: body.available_until,
            free_badge_id: free_badge_id.into_inner()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let mapped_free_badge = FreeBadgePresenter::to_http(result.unwrap());

        return HttpResponse::Ok().json(JsonWrappedEntity {
            data: mapped_free_badge
        });
    }

    async fn delete(user: web::ReqData<ReqUser>, free_badge_id: web::Path<Uuid>) -> impl Responder {
        let service = match delete_free_badge_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(DeleteFreeBadgeParams {
            free_badge_id: free_badge_id.into_inner(),
            user_role: user.into_inner().user_role.unwrap()
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }
}
