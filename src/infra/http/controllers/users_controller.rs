use actix_web::{http::StatusCode, web, HttpResponse, HttpResponseBuilder, Responder};
use either::Either::*;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::role::Role;
use crate::domain::factories::{
    change_password_service_factory, create_user_service_factory, fetch_many_users_service_factory,
    get_user_service_factory, update_user_service_factory,
};
use crate::domain::repositories::user_repository::UserQueryType;
use crate::domain::services::change_password_service::ChangePasswordParams;
use crate::domain::services::create_user_service::CreateUserParams;
use crate::domain::services::fetch_many_users_service::FetchManyUsersParams;
use crate::domain::services::get_user_service::GetUserServiceParams;
use crate::domain::services::update_user_service::UpdateUserParams;
use crate::infra::http::dtos::change_password::ChangePasswordDto;
use crate::infra::http::dtos::create_user::CreateUserDto;
use crate::infra::http::dtos::list_users::ListUsersDto;
use crate::infra::http::dtos::update_user::UpdateUserDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::AuthenticationMiddleware;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::http::presenters::user::UserPresenter;

use super::controller::ControllerTrait;

pub struct UsersController;

impl ControllerTrait for UsersController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/users")
                // CREATE
                .route("/new", web::post().to(Self::create))
                // UPDATE
                .route(
                    "/{id}/update",
                    web::put().to(Self::update).wrap(AuthenticationMiddleware),
                )
                // CHANGE USER'S PASSWORD
                .route(
                    "/password",
                    web::put()
                        .to(Self::edit_password)
                        .wrap(AuthenticationMiddleware),
                )
                // LIST USERS WITH PAGINATION
                .route(
                    "/list",
                    web::get().to(Self::list).wrap(AuthenticationMiddleware),
                )
                // GET SINGLE USER BY ID
                .route(
                    "/{id}",
                    web::get().to(Self::get).wrap(AuthenticationMiddleware),
                ),
        );
    }
}

impl UsersController {
    async fn create(body: web::Json<CreateUserDto>) -> impl Responder {
        if let Err(e) = body.validate() {
            return HttpResponse::BadRequest()
                .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
        };

        let create_user_service = match create_user_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let CreateUserDto { nickname, password } = body.into_inner();

        let result = create_user_service
            .exec(CreateUserParams { nickname, password })
            .await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(ErrorPresenter::to_http(err));
        }

        let user = result.unwrap();
        let mapped_user = UserPresenter::to_http(user);

        HttpResponse::Created().json(json!({"user": mapped_user}))
    }

    async fn update(
        body: web::Json<UpdateUserDto>,
        user_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>,
    ) -> impl Responder {
        if let Err(e) = body.validate() {
            return HttpResponse::BadRequest()
                .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
        };

        let update_user_service = match update_user_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let UpdateUserDto {
            nickname,
            password,
            role,
        } = body.into_inner();

        let role = {
            match role {
                None => None,
                Some(role) => {
                    let parsed_role = Role::from_str(&role);

                    if parsed_role.is_err() {
                        let err = parsed_role.unwrap_err();
                        return HttpResponseBuilder::new(
                            StatusCode::from_u16(err.code().to_owned()).unwrap(),
                        )
                        .json(json!({"error": err.message()}));
                    }

                    Some(parsed_role.unwrap())
                }
            }
        };

        let ReqUser {
            user_id: staff_id,
            user_role: staff_role,
            ..
        } = user.into_inner();

        let result = update_user_service
            .exec(UpdateUserParams {
                user_id: user_id.into_inner(),
                nickname,
                password,
                role,
                staff_id,
                staff_role: staff_role.unwrap(),
            })
            .await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(ErrorPresenter::to_http(err));
        }

        let user = result.unwrap();
        let mapped_user = UserPresenter::to_http(user);

        HttpResponse::Ok().json(json!({"user": mapped_user}))
    }

    async fn edit_password(
        body: web::Json<ChangePasswordDto>,
        user: web::ReqData<ReqUser>,
    ) -> impl Responder {
        if let Err(e) = body.validate() {
            return HttpResponse::BadRequest()
                .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
        };

        let change_password_service = match change_password_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let ChangePasswordDto {
            current_password,
            new_password,
        } = body.into_inner();

        let result = change_password_service
            .exec(ChangePasswordParams {
                current_password,
                new_password,
                user_id: user.user_id,
            })
            .await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(ErrorPresenter::to_http(err));
        }

        HttpResponse::Ok().finish()
    }

    async fn get(user_id: web::Path<Uuid>) -> impl Responder {
        let get_user_service = match get_user_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = get_user_service
            .exec(GetUserServiceParams {
                user_id: user_id.into_inner(),
            })
            .await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(ErrorPresenter::to_http(err));
        }

        let mapped_user = {
            let user = result.unwrap();

            user.map(UserPresenter::to_http)
        };

        HttpResponse::Ok().json(json!({"user": mapped_user}))
    }

    async fn list(query: web::Query<ListUsersDto>) -> impl Responder {
        if let Err(e) = query.validate() {
            return HttpResponse::BadRequest()
                .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
        };

        let fetch_many_users_service = match fetch_many_users_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let ListUsersDto {
            nickname,
            page,
            per_page,
            role,
        } = query.into_inner();

        let query: Option<UserQueryType>;

        if nickname.is_some() {
            query = Some(UserQueryType::Nickname(nickname.unwrap()));
        } else if role.is_none() {
            query = None;
        } else {
            let parsed_role = Role::from_str(role.unwrap().as_str());

            if parsed_role.is_err() {
                let err = parsed_role.unwrap_err();

                return HttpResponseBuilder::new(
                    StatusCode::from_u16(err.code().to_owned()).unwrap(),
                )
                .json(json!({"error": err.message()}));
            }

            query = Some(UserQueryType::Role(parsed_role.unwrap()));
        }

        let result = fetch_many_users_service
            .exec(FetchManyUsersParams {
                page,
                per_page: per_page.map(|v| v as u32),
                query,
            })
            .await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(ErrorPresenter::to_http(err));
        }

        let result = result.unwrap();
        let mut mapped_users = Vec::new();

        for user in result.data.into_iter() {
            mapped_users.push(UserPresenter::to_http(user));
        }

        HttpResponse::Ok().json(json!({
            "pagination": PaginationPresenter::to_http(result.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE) ),
            "data": mapped_users
        }))
    }
}
