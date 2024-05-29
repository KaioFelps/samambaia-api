use std::str::FromStr;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder};
use actix_web_lab::middleware::from_fn;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::domain::domain_entities::role::Role;
use crate::domain::factories::{change_password_service_factory, create_user_service_factory, update_user_service_factory};
use crate::domain::services::change_password_service::ChangePasswordParams;
use crate::domain::services::create_user_service::CreateUserParams;
use crate::domain::services::update_user_service::UpdateUserParams;
use crate::infra::http::dtos::change_password::ChangePasswordDto;
use crate::infra::http::dtos::create_user::CreateUserDto;
use crate::infra::http::dtos::update_user::UpdateUserDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::user::UserPresenter;

use super::controller::ControllerTrait;

pub struct UsersController;

impl ControllerTrait for UsersController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/users")
            // CREATE
            .route("/new", web::post().to(Self::create))

            // UPDATE
            .route(
                "/{id}/update",
                web::put()
                .to(Self::update)
                .wrap(from_fn(authentication_middleware))
            )

            // CHANGE USER'S PASSWORD
            .route(
                "/password",
                web::put()
                .to(Self::edit_password)
                .wrap(from_fn(authentication_middleware))
            )
        );
    }
}

impl UsersController {
    async fn create(
        body: web::Json<CreateUserDto>,
    ) -> impl Responder {
        match body.validate() {
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
            },
            Ok(()) => ()
        };

        let create_user_service = create_user_service_factory::exec().await;

        let CreateUserDto { nickname, password } = body.into_inner();

        let result =
            create_user_service.exec(CreateUserParams { nickname, password }).await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
            .json(ErrorPresenter::to_http(err));
        }

        let user = result.unwrap();
        let mapped_user = UserPresenter::to_http(user);

        return HttpResponse::Created().json(json!({"user": mapped_user}));
    }

    async fn update(
        body: web::Json<UpdateUserDto>,
        user_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>
    ) -> impl Responder {
        match body.validate() {
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
            },
            Ok(()) => ()
        };

        let update_user_service = update_user_service_factory::exec().await;

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
                        return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
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

        let result =
            update_user_service.exec(UpdateUserParams {
                user_id: user_id.into_inner(),
                nickname,
                password,
                role,
                staff_id,
                staff_role: staff_role.unwrap()
            }).await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
            .json(ErrorPresenter::to_http(err));
        }

        let user = result.unwrap();
        let mapped_user = UserPresenter::to_http(user);

        return HttpResponse::Ok().json(json!({"user": mapped_user}));
    }

    async fn edit_password(
        body: web::Json<ChangePasswordDto>,
        user: web::ReqData<ReqUser>
    ) -> impl Responder {
        match body.validate() {
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
            },
            Ok(()) => ()
        };

        let change_password_service = change_password_service_factory::exec().await;

        let ChangePasswordDto { current_password, new_password } = body.into_inner();

        let result = change_password_service.exec(ChangePasswordParams {
            current_password,
            new_password,
            user_id: user.user_id,
        }).await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
            .json(ErrorPresenter::to_http(err));
        }

        return HttpResponse::Ok().finish();
    }
}
