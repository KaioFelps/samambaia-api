use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder};
use actix_web_lab::middleware::from_fn;
use serde_json::json;
use validator::Validate;

use crate::domain::factories::create_user_service_factory;
use crate::domain::services::create_user_service::CreateUserParams;
use crate::errors::internal_error::InternalError;
use crate::errors::user_already_exists_error::UserAlreadyExistsError;
use crate::infra::http::dtos::create_user::CreateUserDto;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenter::user::UserPresenter;

pub struct UsersController {}

impl UsersController {
    pub fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/users")
            // CREATE
            .route("/new", web::post().to(Self::create))

            // UPDATE
            .route("/{id}/update", web::put().to(Self::update).wrap(from_fn(authentication_middleware)))
        );
    }

    async fn create(
        body: web::Json<CreateUserDto>,
    ) -> impl Responder {
        match body.validate() {
            Err(e) => {
                return HttpResponse::BadRequest().json(json!({
                    "error": e.field_errors()
                }));
            },
            Ok(()) => ()
    };

        let create_user_service = create_user_service_factory::exec().await;

        let CreateUserDto { nickname, password } = body.into_inner();

        let result =
            create_user_service.exec(CreateUserParams { nickname, password }).await;

        if result.is_err() {
            let err = result.unwrap_err();

            if err.downcast_ref::<UserAlreadyExistsError>().is_some() {
                let err = err.downcast::<UserAlreadyExistsError>().unwrap();

                return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(json!({"error": err.message()}));
            }

            let err = err.downcast::<InternalError>().unwrap();
            
            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
            .json(json!({"error": err.message()}));
        }

        let user = result.unwrap();
        let mapped_user = UserPresenter::to_http(user);

        return HttpResponse::Created().json(json!({"user": mapped_user}));
    }

    async fn update() -> impl Responder {
        HttpResponse::Ok().json("")
    }
}
