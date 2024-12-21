use actix_web::cookie::Cookie;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, EncodingKey};
use log::info;
use serde_json::json;
use validator::Validate;

use crate::domain::factories::authenticate_user_service_factory;
use crate::domain::services::authenticate_user_service::AuthenticateUserParams;
use crate::error::DomainError;
use crate::infra::extensions::validator::IntoDomainError;
use crate::infra::http::dtos::login::LoginDto;
use crate::infra::jwt::jwt_service::{DecodedToken, JwtService, MakeJwtResult};
use crate::infra::sea::sea_service::SeaService;
use crate::ENV_VARS;

use super::controller::ControllerTrait;
use super::AppResponse;

pub struct SessionsController;

impl ControllerTrait for SessionsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/session")
                .route("/login", web::post().to(Self::login))
                .route("/refresh", web::post().to(Self::refresh))
                .route("/logout", web::post().to(Self::logout)),
        );
    }
}

impl SessionsController {
    async fn login(db_conn: web::Data<SeaService>, body: web::Json<LoginDto>) -> AppResponse {
        let LoginDto { nickname, password } = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoDomainError::into_domain_err)?;

        let authenticate_service = authenticate_user_service_factory::exec(&db_conn);

        let MakeJwtResult {
            access_token,
            refresh_token,
        } = authenticate_service
            .exec(AuthenticateUserParams { nickname, password })
            .await?;

        let refresh_cookie = Cookie::build("refresh_token", refresh_token.token)
            .domain(&ENV_VARS.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

        Ok(HttpResponse::Ok().cookie(refresh_cookie).json(json!({
            "accessToken": access_token.token,
        })))
    }

    async fn refresh(request: HttpRequest) -> AppResponse {
        let refresh_token = request.cookie("refresh_token");

        if refresh_token.is_none() {
            info!("Refresh token is None; bad request.");
            return Err(DomainError::bad_request_err());
        }

        let refresh_token = refresh_token.unwrap();
        let refresh_token = refresh_token.value();

        let jwt_service = JwtService {};
        let decoded_token = jwt_service.decode_jwt(
            refresh_token.into(),
            DecodingKey::from_secret(ENV_VARS.jwt_secret.as_ref()),
        );

        if let Err(err) = decoded_token {
            return Err(match err.kind() {
                ErrorKind::InvalidToken
                | ErrorKind::InvalidSignature
                | ErrorKind::MissingRequiredClaim(_)
                | ErrorKind::ExpiredSignature
                | ErrorKind::InvalidIssuer
                | ErrorKind::InvalidAudience
                | ErrorKind::InvalidSubject
                | ErrorKind::ImmatureSignature
                | ErrorKind::Json(_)
                | ErrorKind::Utf8(_) => {
                    info!("Token decoding validation error; bad request.");
                    DomainError::bad_request_err()
                }
                _ => {
                    info!("Token decoding configuration error; internal server error.");
                    DomainError::bad_request_err()
                }
            });
        }

        let DecodedToken {
            user_role, user_id, ..
        } = decoded_token.unwrap();

        if user_role.is_none() {
            info!("User role from decoded jwt token is None; bad request.");
            return Err(DomainError::bad_request_err());
        }

        let tokens = jwt_service.make_jwt(
            user_id,
            user_role.unwrap(),
            EncodingKey::from_secret(ENV_VARS.jwt_secret.as_ref()),
        );

        if tokens.is_err() {
            info!("Failed to make new jwt token; internal server error.");
            return Err(DomainError::internal_err());
        }

        let MakeJwtResult {
            access_token,
            refresh_token,
        } = tokens.unwrap();

        let refresh_cookie = Cookie::build("refresh_token", refresh_token.token)
            .domain(&ENV_VARS.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

        Ok(HttpResponse::Ok().cookie(refresh_cookie).json(json!({
            "accessToken": access_token.token,
        })))
    }

    async fn logout() -> impl Responder {
        let mut refresh_cookie = Cookie::build("refresh_token", "")
            .domain(&ENV_VARS.domain)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();

        refresh_cookie.make_removal();

        HttpResponse::Ok().cookie(refresh_cookie).finish()
    }
}
