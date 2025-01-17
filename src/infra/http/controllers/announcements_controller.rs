use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::announcements::{
    create_announcement_service_factory, delete_announcement_service_factory,
    fetch_many_announcements_service_factory, update_announcement_service_factory,
};
use crate::domain::services::announcements::create_announcement_service::CreateAnnouncementParams;
use crate::domain::services::announcements::delete_announcement_service::DeleteAnnouncementParams;
use crate::domain::services::announcements::fetch_many_announcements_service::{
    FetchManyAnnouncementsParams, FetchManyAnnouncementsResponse,
};
use crate::domain::services::announcements::update_announcement_service::UpdateAnnouncementParams;
use crate::infra::extensions::validator::IntoSamambaiaError;
use crate::infra::http::dtos::announcements::{
    CreateAnnouncementDto, ListAnnouncementsDto, UpdateAnnouncementDto,
};
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::AuthenticationMiddleware;
use crate::infra::http::presenters::announcement::{AnnouncementPresenter, MappedAnnouncement};
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::{
    JsonWrappedEntity, JsonWrappedPaginatedEntity, PresenterTrait,
};
use crate::infra::sea::sea_service::SeaService;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use super::controller::ControllerTrait;
use super::AppResponse;

pub struct AnnouncementsController;

impl ControllerTrait for AnnouncementsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/announcements")
                // CREATE
                .route(
                    "/new",
                    web::post().to(Self::create).wrap(AuthenticationMiddleware),
                )
                // READ
                .route("/list", web::get().to(Self::list))
                // UPDATE
                .route(
                    "/{id}/update",
                    web::put().to(Self::update).wrap(AuthenticationMiddleware),
                )
                // DELETE
                .route(
                    "/{id}/delete",
                    web::delete()
                        .to(Self::delete)
                        .wrap(AuthenticationMiddleware),
                ),
        );
    }
}

impl AnnouncementsController {
    async fn create(
        db_conn: web::Data<SeaService>,
        body: web::Json<CreateAnnouncementDto>,
        staff: web::ReqData<ReqUser>,
    ) -> AppResponse {
        body.validate()
            .map_err(IntoSamambaiaError::into_domain_err)?;
        let body = body.into_inner();

        let service = create_announcement_service_factory::exec(&db_conn);
        service
            .exec(CreateAnnouncementParams {
                description: body.description,
                external: body.external,
                image: body.image,
                url: body.url,
                staff_id: staff.user_id,
            })
            .await
            .map(|announcement| {
                HttpResponse::Created().json(JsonWrappedEntity {
                    data: AnnouncementPresenter::to_http(announcement),
                })
            })
    }

    async fn list(
        db_conn: web::Data<SeaService>,
        query: web::Query<ListAnnouncementsDto>,
    ) -> AppResponse {
        let service = fetch_many_announcements_service_factory::exec(&db_conn);
        service
            .exec(FetchManyAnnouncementsParams {
                page: query.page,
                per_page: query.per_page.map(|p| p as u32),
                query: query.description.to_owned(),
            })
            .await
            .map(|FetchManyAnnouncementsResponse { data, pagination }| {
                let mapped_announcements = data
                    .into_iter()
                    .map(AnnouncementPresenter::to_http)
                    .collect::<Vec<MappedAnnouncement>>();

                HttpResponse::Ok().json(JsonWrappedPaginatedEntity {
                    data: mapped_announcements,
                    pagination: PaginationPresenter::to_http(
                        pagination,
                        query.per_page.unwrap_or(DEFAULT_PER_PAGE),
                    ),
                })
            })
    }

    async fn update(
        db_conn: web::Data<SeaService>,
        body: web::Json<UpdateAnnouncementDto>,
        staff: web::ReqData<ReqUser>,
        announcement_id: web::Path<Uuid>,
    ) -> AppResponse {
        body.validate()
            .map_err(IntoSamambaiaError::into_domain_err)?;
        let body = body.into_inner();

        let service = update_announcement_service_factory::exec(&db_conn);

        service
            .exec(UpdateAnnouncementParams {
                announcement_id: announcement_id.into_inner(),
                user_id: staff.user_id,
                description: body.description,
                external: body.external,
                image: body.image,
                url: body.url,
            })
            .await
            .map(|announcement| {
                HttpResponse::Ok().json(JsonWrappedEntity {
                    data: AnnouncementPresenter::to_http(announcement),
                })
            })
    }

    async fn delete(
        db_conn: web::Data<SeaService>,
        staff: web::ReqData<ReqUser>,
        announcement_id: web::Path<Uuid>,
    ) -> AppResponse {
        let service = delete_announcement_service_factory::exec(&db_conn);

        service
            .exec(DeleteAnnouncementParams {
                announcement_id: &announcement_id,
                user_id: &staff.user_id,
            })
            .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
