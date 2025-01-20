use super::controller::ControllerTrait;
use super::AppResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::journalism::free_badges::{
    create_free_badge_service_factory, delete_free_badge_service_factory,
    fetch_many_free_badges_service_factory, update_free_badge_service_factory,
};
use crate::domain::services::journalism::free_badges::{
    create_free_badge_service::CreateFreeBadgeParams,
    delete_free_badge_service::DeleteFreeBadgeParams,
    fetch_many_free_badges_service::FetchManyFreeBadgesParams,
    update_free_badge_service::UpdateFreeBadgeParams,
};
use crate::error::IntoSamambaiaError;
use crate::infra::http::dtos::create_free_badge::CreateFreeBadgeDto;
use crate::infra::http::dtos::simple_pagination_query::SimplePaginationQueryDto;
use crate::infra::http::dtos::update_free_badge::UpdateFreeBadgeDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::free_badge::FreeBadgePresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::infra::sea::sea_service::SeaService;
use actix_web::{middleware::from_fn, web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

pub struct FreeBadgesController;

impl ControllerTrait for FreeBadgesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/free_badges")
                // CREATE
                .route(
                    "/new",
                    web::post()
                        .to(Self::create)
                        .wrap(from_fn(authentication_middleware)),
                )
                // READ
                .route("/list", web::get().to(Self::list))
                // UPDATE
                .route(
                    "/{id}/update",
                    web::put()
                        .to(Self::update)
                        .wrap(from_fn(authentication_middleware)),
                )
                // DELETE
                .route(
                    "/{id}/delete",
                    web::delete()
                        .to(Self::delete)
                        .wrap(from_fn(authentication_middleware)),
                ),
        );
    }
}

impl FreeBadgesController {
    async fn create(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        body: web::Json<CreateFreeBadgeDto>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let service = create_free_badge_service_factory::exec(&db_conn);

        let free_badge = service
            .exec(CreateFreeBadgeParams {
                user_role: user.into_inner().user_role.unwrap(),
                code: body.code,
                image: body.image,
                link: body.link,
                link_is_external: body.link_is_external,
                available_until: body.available_until,
            })
            .await?;

        Ok(HttpResponse::Created().json(JsonWrappedEntity {
            data: FreeBadgePresenter::to_http(free_badge),
        }))
    }

    async fn list(
        db_conn: web::Data<SeaService>,
        query: web::Query<SimplePaginationQueryDto>,
    ) -> AppResponse {
        let service = fetch_many_free_badges_service_factory::exec(&db_conn);

        let free_badges = service
            .exec(FetchManyFreeBadgesParams {
                page: query.page,
                per_page: if query.per_page.is_some() {
                    Some(query.page.unwrap())
                } else {
                    None
                },
            })
            .await?;

        Ok(
            HttpResponse::Ok().json(FreeBadgePresenter::to_json_paginated_wrapper(
                free_badges.data,
                free_badges.pagination,
                query.per_page.unwrap_or(DEFAULT_PER_PAGE),
            )),
        )
    }

    async fn update(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        body: web::Json<UpdateFreeBadgeDto>,
        free_badge_id: web::Path<Uuid>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let service = update_free_badge_service_factory::exec(&db_conn);

        let free_badge = service
            .exec(UpdateFreeBadgeParams {
                user_role: user.into_inner().user_role.unwrap(),
                code: body.code,
                image: body.image,
                link: body.link,
                link_is_external: body.link_is_external,
                available_until: body.available_until,
                free_badge_id: free_badge_id.into_inner(),
            })
            .await?;

        let mapped_free_badge = FreeBadgePresenter::to_http(free_badge);

        Ok(HttpResponse::Ok().json(JsonWrappedEntity {
            data: mapped_free_badge,
        }))
    }

    async fn delete(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        free_badge_id: web::Path<Uuid>,
    ) -> AppResponse {
        let service = delete_free_badge_service_factory::exec(&db_conn);

        service
            .exec(DeleteFreeBadgeParams {
                free_badge_id: free_badge_id.into_inner(),
                user_role: user.into_inner().user_role.unwrap(),
            })
            .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
