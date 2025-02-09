use std::str::FromStr;

use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use super::controller::ControllerTrait;
use super::AppResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::role::Role;
use crate::domain::factories::identity::{
    change_password_service_factory,
    create_user_service_factory,
    fetch_many_users_service_factory,
    get_user_service_factory,
    update_user_service_factory,
};
use crate::domain::repositories::user_repository::UserQueryType;
use crate::domain::services::identity::change_password_service::ChangePasswordParams;
use crate::domain::services::identity::create_user_service::CreateUserParams;
use crate::domain::services::identity::fetch_many_users_service::FetchManyUsersParams;
use crate::domain::services::identity::get_user_service::GetUserServiceParams;
use crate::domain::services::identity::update_user_service::UpdateUserParams;
use crate::error::IntoSamambaiaError;
use crate::infra::http::dtos::change_password::ChangePasswordDto;
use crate::infra::http::dtos::create_user::CreateUserDto;
use crate::infra::http::dtos::list_users::ListUsersDto;
use crate::infra::http::dtos::update_user::UpdateUserDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::AuthenticationMiddleware;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::http::presenters::user::{MappedUser, UserPresenter};
use crate::infra::sea::sea_service::SeaService;

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
    async fn create(db_conn: web::Data<SeaService>, body: web::Json<CreateUserDto>) -> AppResponse {
        let CreateUserDto { nickname, password } = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;
        let create_user_service = create_user_service_factory::exec(&db_conn);

        let user = create_user_service
            .exec(CreateUserParams { nickname, password })
            .await?;

        let mapped_user = UserPresenter::to_http(user);

        Ok(HttpResponse::Created().json(json!({"user": mapped_user})))
    }

    async fn update(
        db_conn: web::Data<SeaService>,
        body: web::Json<UpdateUserDto>,
        user_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>,
    ) -> AppResponse {
        let UpdateUserDto {
            nickname,
            password,
            role,
        } = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let update_user_service = update_user_service_factory::exec(&db_conn);

        let role = match role.map(|role| Role::from_str(&role)) {
            Some(role) => match role {
                Err(err) => return Err(err),
                Ok(role) => Some(role),
            },
            None => None,
        };

        let ReqUser {
            user_id: staff_id,
            user_role: staff_role,
            ..
        } = user.into_inner();

        let user = update_user_service
            .exec(UpdateUserParams {
                user_id: user_id.into_inner(),
                nickname,
                password,
                role,
                staff_id,
                staff_role: staff_role.unwrap(),
            })
            .await?;

        let mapped_user = UserPresenter::to_http(user);

        Ok(HttpResponse::Ok().json(json!({"user": mapped_user})))
    }

    async fn edit_password(
        db_conn: web::Data<SeaService>,
        body: web::Json<ChangePasswordDto>,
        user: web::ReqData<ReqUser>,
    ) -> AppResponse {
        let ChangePasswordDto {
            current_password,
            new_password,
        } = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let change_password_service = change_password_service_factory::exec(&db_conn);

        change_password_service
            .exec(ChangePasswordParams {
                current_password,
                new_password,
                user_id: user.user_id,
            })
            .await?;

        Ok(HttpResponse::Ok().finish())
    }

    async fn get(db_conn: web::Data<SeaService>, user_id: web::Path<Uuid>) -> AppResponse {
        let get_user_service = get_user_service_factory::exec(&db_conn);

        let user = get_user_service
            .exec(GetUserServiceParams {
                user_id: user_id.into_inner(),
            })
            .await?
            .map(UserPresenter::to_http);

        Ok(HttpResponse::Ok().json(json!({"user": user})))
    }

    async fn list(db_conn: web::Data<SeaService>, query: web::Query<ListUsersDto>) -> AppResponse {
        let ListUsersDto {
            nickname,
            page,
            per_page,
            role,
        } = query
            .validate()
            .map(|_| query.into_inner())
            .map_err(IntoSamambaiaError::into_samambaia_error)?;

        let fetch_many_users_service = fetch_many_users_service_factory::exec(&db_conn);

        let query: Option<UserQueryType> = if let Some(nickname) = nickname {
            Some(UserQueryType::Nickname(nickname))
        } else {
            match role.map(|role| Role::from_str(&role)) {
                Some(role) => match role {
                    Err(err) => return Err(err),
                    Ok(role) => Some(UserQueryType::Role(role)),
                },
                None => None,
            }
        };

        let users = fetch_many_users_service
            .exec(FetchManyUsersParams {
                page,
                per_page: per_page.map(|v| v as u32),
                query,
            })
            .await?;

        let mapped_users = users
            .data
            .into_iter()
            .map(UserPresenter::to_http)
            .collect::<Vec<MappedUser>>();

        Ok(HttpResponse::Ok().json(json!({
            "pagination": PaginationPresenter::to_http(users.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE) ),
            "data": mapped_users
        })))
    }
}
