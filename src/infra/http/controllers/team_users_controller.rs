use super::controller::ControllerTrait;
use super::AppResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::{
    create_team_user_service_factory, delete_team_user_service_factory,
    fetch_many_team_users_service_factory, update_team_user_service_factory,
};
use crate::domain::repositories::team_user_repository::TeamUserQueryType;
use crate::domain::services::create_team_user_service::CreateTeamUserParams;
use crate::domain::services::delete_team_user_service::DeleteTeamUserParams;
use crate::domain::services::fetch_many_team_users_service::FetchManyTeamUsersParams;
use crate::domain::services::update_team_user_service::UpdateTeamUserParams;
use crate::infra::extensions::validator::IntoDomainError;
use crate::infra::http::dtos::create_team_user::CreateTeamUserDto;
use crate::infra::http::dtos::list_team_user::ListTeamUsersDto;
use crate::infra::http::dtos::update_team_user::UpdateTeamUserDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::infra::http::presenters::team_user::{MappedTeamUser, TeamUserPresenter};
use actix_web::{middleware::from_fn, web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

pub struct TeamUsersController;

impl ControllerTrait for TeamUsersController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/team_users")
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

impl TeamUsersController {
    async fn create(
        body: web::Json<CreateTeamUserDto>,
        user: web::ReqData<ReqUser>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoDomainError::into_domain_err)?;

        let service = create_team_user_service_factory::exec().await?;

        let team_user = service
            .exec(CreateTeamUserParams {
                nickname: body.nickname,
                team_role_id: body.team_role_id,
                staff_role: user.into_inner().user_role.unwrap(),
                discord: body.discord,
                twitter: body.twitter,
                user_function: body.user_function,
            })
            .await?;

        let mapped_team_user = TeamUserPresenter::to_http(team_user);

        Ok(HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_team_user,
        }))
    }

    async fn list(query: web::Query<ListTeamUsersDto>) -> AppResponse {
        let service = fetch_many_team_users_service_factory::exec().await?;

        let ListTeamUsersDto {
            page,
            per_page,
            team_role_id,
            nickname,
        } = query.into_inner();

        let query = {
            if let Some(team_role_id) = team_role_id {
                Some(TeamUserQueryType::TeamRole(team_role_id))
            } else {
                nickname.map(TeamUserQueryType::Nickname)
            }
        };

        let team_users = service
            .exec(FetchManyTeamUsersParams {
                per_page: per_page.map(|pp| pp as u32),
                query,
                page,
            })
            .await?;

        let mapped_team_users = team_users
            .data
            .into_iter()
            .map(TeamUserPresenter::to_http)
            .collect::<Vec<MappedTeamUser>>();

        let mapped_pagination = PaginationPresenter::to_http(
            team_users.pagination,
            per_page.unwrap_or(DEFAULT_PER_PAGE),
        );

        Ok(
            HttpResponse::Ok().json(TeamUserPresenter::to_json_paginated_wrapper(
                mapped_team_users,
                mapped_pagination,
            )),
        )
    }

    async fn update(
        body: web::Json<UpdateTeamUserDto>,
        user: web::ReqData<ReqUser>,
        team_user_id: web::Path<Uuid>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoDomainError::into_domain_err)?;

        let service = update_team_user_service_factory::exec().await?;

        let team_user = service
            .exec(UpdateTeamUserParams {
                nickname: body.nickname,
                team_role_id: body.team_role_id,
                twitter: body.twitter,
                discord: body.discord,
                user_function: body.user_function,
                staff_role: user.into_inner().user_role.unwrap(),
                team_user_id: team_user_id.into_inner(),
            })
            .await?;

        let mapped_team_user = TeamUserPresenter::to_http(team_user);

        Ok(HttpResponse::Ok().json(JsonWrappedEntity {
            data: mapped_team_user,
        }))
    }

    async fn delete(user: web::ReqData<ReqUser>, team_user_id: web::Path<Uuid>) -> AppResponse {
        let service = delete_team_user_service_factory::exec().await?;

        service
            .exec(DeleteTeamUserParams {
                staff_role: user.into_inner().user_role.unwrap(),
                team_user_id: team_user_id.into_inner(),
            })
            .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
