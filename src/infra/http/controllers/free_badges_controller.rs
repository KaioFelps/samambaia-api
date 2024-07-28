use actix_web::{web, HttpResponse, Responder};
use either::{Left, Right};
use validator::Validate;
use super::controller::ControllerTrait;
use crate::domain::factories::{
    create_free_badge_service_factory,
    fetch_many_free_badges_service_factory,
    update_free_badge_service_factory,
    delete_free_badge_service_factory
};
use crate::domain::services::create_free_badge_service::CreateFreeBadgeParams;
use crate::infra::http::dtos::create_free_badge::CreateFreeBadgeDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::free_badge::FreeBadgePresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::util::generate_error_response;

pub struct FreeBadgesController;

impl ControllerTrait for FreeBadgesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/free_badges")
            // CREATE
            .route("/new", web::post().to(Self::create))

            // READ
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update))

            // DELETE
            .route("/{id}/delete", web::delete().to(Self::delete))
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
