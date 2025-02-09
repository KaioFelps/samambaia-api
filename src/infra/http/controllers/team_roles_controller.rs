use actix_web::middleware::from_fn;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use super::controller::ControllerTrait;
use super::AppResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::teams::{
    create_team_role_service_factory,
    delete_team_role_service_factory,
    fetch_many_team_roles_service_factory,
    update_team_role_service_factory,
};
use crate::domain::repositories::team_role_repository::TeamRoleQueryType;
use crate::domain::services::teams::create_team_role_service::CreateTeamRoleParams;
use crate::domain::services::teams::delete_team_role_service::DeleteTeamRoleParams;
use crate::domain::services::teams::fetch_many_team_roles_service::FetchManyTeamRolesParams;
use crate::domain::services::teams::update_team_role_service::UpdateTeamRoleParams;
use crate::error::IntoSamambaiaError;
use crate::infra::http::dtos::create_team_role::CreateTeamRoleDto;
use crate::infra::http::dtos::list_team_role::ListTeamRoleDto;
use crate::infra::http::dtos::update_team_role::UpdateTeamRoleDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::infra::http::presenters::team_role::TeamRolePresenter;
use crate::infra::sea::sea_service::SeaService;

pub struct TeamRolesController;

impl ControllerTrait for TeamRolesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/team_roles")
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

impl TeamRolesController {
    async fn create(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        body: web::Json<CreateTeamRoleDto>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let service = create_team_role_service_factory::exec(&db_conn);

        let team_role = service
            .exec(CreateTeamRoleParams {
                staff_role: user.into_inner().user_role.unwrap(),
                title: body.title,
                description: body.description,
            })
            .await?;

        let mapped_team_role = TeamRolePresenter::to_http(team_role);

        Ok(HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_team_role,
        }))
    }

    async fn list(
        db_conn: web::Data<SeaService>,
        query: web::Query<ListTeamRoleDto>,
    ) -> AppResponse {
        let service = fetch_many_team_roles_service_factory::exec(&db_conn);

        let query = query
            .validate()
            .map(|_| query.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let team_roles = service
            .exec(FetchManyTeamRolesParams {
                per_page: if query.per_page.is_some() {
                    Some(query.per_page.unwrap() as u32)
                } else {
                    None
                },
                page: query.page,
                query: query.title.map(TeamRoleQueryType::Title),
            })
            .await?;

        Ok(
            HttpResponse::Ok().json(TeamRolePresenter::to_json_paginated_wrapper(
                team_roles.data,
                team_roles.pagination,
                query.per_page.unwrap_or(DEFAULT_PER_PAGE),
            )),
        )
    }

    async fn update(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        team_role_id: web::Path<Uuid>,
        body: web::Json<UpdateTeamRoleDto>,
    ) -> AppResponse {
        let UpdateTeamRoleDto { title, description } =
            body.validate()
                .map(|_| body.into_inner())
                .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let service = update_team_role_service_factory::exec(&db_conn);

        let user = user.into_inner();

        let team_role = service
            .exec(UpdateTeamRoleParams {
                title,
                staff_role: user.user_role.unwrap(),
                description,
                team_role_id: team_role_id.into_inner(),
            })
            .await?;

        let mapped_team_role = TeamRolePresenter::to_http(team_role);

        Ok(HttpResponse::Ok().json(JsonWrappedEntity {
            data: mapped_team_role,
        }))
    }

    async fn delete(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        team_role_id: web::Path<Uuid>,
    ) -> AppResponse {
        let service = delete_team_role_service_factory::exec(&db_conn);

        service
            .exec(DeleteTeamRoleParams {
                team_role_id: team_role_id.into_inner(),
                staff_role: user.into_inner().user_role.unwrap(),
            })
            .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
