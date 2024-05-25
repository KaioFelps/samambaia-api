use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder, HttpRequest};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, EncodingKey};
use log::info;
use serde_json::json;
use validator::Validate;

use crate::domain::factories::authenticate_user_service_factory;
use crate::domain::services::authenticate_user_service::AuthenticateUserParams;
use crate::infra::http::dtos::login::LoginDto;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::jwt::jwt_service::{DecodedToken, JwtService, MakeJwtResult};
use crate::ENV_VARS;

use super::controller::ControllerTrait;

pub struct SessionsController;

impl ControllerTrait for SessionsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/session")
            .route("/login", web::post().to(Self::login))
            .route("/refresh", web::post().to(Self::refresh))
            .route("/logout", web::post().to(Self::logout))
        );
    }
}

impl SessionsController {
    async fn login(
        body: web::Json<LoginDto>,
    ) -> impl Responder {
        match body.validate() {
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(e.field_errors()));
            },
            Ok(()) => ()
        };

        let authenticate_service = authenticate_user_service_factory::exec().await;

        let LoginDto { nickname, password } = body.into_inner();

        let result =
            authenticate_service.exec(AuthenticateUserParams { nickname, password }).await;

        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
                .json(ErrorPresenter::to_http(err));
        }

        let MakeJwtResult { access_token, refresh_token } = result.unwrap();

        let refresh_cookie = Cookie::build("refresh_token", refresh_token.token)
            .domain(&ENV_VARS.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

        return HttpResponse::Ok()
            .cookie(refresh_cookie)
            .json(json!({
                "accessToken": access_token.token,
            }));
    }

    async fn refresh(request: HttpRequest) -> impl Responder {
        let refresh_token = request.cookie("refresh_token");

        if refresh_token.is_none() {
            info!("Refresh token is None; bad request.");
            return HttpResponse::BadRequest().finish();
        }

        let refresh_token = refresh_token.unwrap();
        let refresh_token = refresh_token.value();

        let jwt_service = JwtService {};
        let decoded_token = jwt_service.decode_jwt(
            refresh_token.into(),
            DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref())
        );

        if decoded_token.is_err() {
            match decoded_token.unwrap_err().kind() {
                ErrorKind::InvalidToken |
                ErrorKind::InvalidSignature |
                ErrorKind::MissingRequiredClaim(_) |
                ErrorKind::ExpiredSignature |
                ErrorKind::InvalidIssuer |
                ErrorKind::InvalidAudience |
                ErrorKind::InvalidSubject |
                ErrorKind::ImmatureSignature |
                ErrorKind::Json(_) |
                ErrorKind::Utf8(_) => {
                    info!("Token decoding validation error; bad request.");
                    return HttpResponse::BadRequest().finish();
                },
                _ => {
                    info!("Token decoding configuration error; internal server error.");
                    return HttpResponse::InternalServerError().finish();
                },
            }
        }

        let DecodedToken { user_role, user_id, .. } = decoded_token.unwrap();

        if user_role.is_none() {
            info!("User role from decoded jwt token is None; bad request.");
            return HttpResponse::BadRequest().finish();
        }

        let tokens = jwt_service.make_jwt(
            user_id,
            user_role.unwrap(),
            EncodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref())
        );
        
        if tokens.is_err() {
            info!("Failed to make new jwt token; internal server error.");
            return HttpResponse::InternalServerError().finish();
        }

        let MakeJwtResult {access_token, refresh_token} = tokens.unwrap();

        let refresh_cookie = Cookie::build("refresh_token", refresh_token.token)
            .domain(&ENV_VARS.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

        return HttpResponse::Ok()
            .cookie(refresh_cookie)
            .json(json!({
                "accessToken": access_token.token,
            }));
    }

    async fn logout() -> impl Responder {
        let mut refresh_cookie = Cookie::build("refresh_token", "")
            .domain(&ENV_VARS.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

        refresh_cookie.make_removal();

        return HttpResponse::Ok().cookie(refresh_cookie).finish();
    }
}
