use crate::configs::app::SESSION_USER_KEY;
use crate::configs::inertia::IntoInertiaRedirect;
use crate::domain::factories::identity::{
    authenticate_user_service_factory, create_user_service_factory,
};
use crate::domain::services::identity::authenticate_user_service::AuthenticateUserParams;
use crate::domain::services::identity::create_user_service::CreateUserParams;
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::{controller::ControllerTrait, AppResponse};
use crate::infra::http::dtos::create_user::CreateUserDto;
use crate::infra::http::dtos::login::LoginDto;
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::infra::sea::sea_service::SeaService;

use actix_session::{Session, SessionExt};
use actix_web::{
    web::{self, Redirect},
    HttpRequest,
};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{hashmap, Inertia, InertiaFacade};

pub struct SessionsController;

impl ControllerTrait for SessionsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/sessions")
                .route("/login", web::post().to(Self::login))
                .route("/register", web::post().to(Self::register_user))
                .route(
                    "/logout",
                    web::post().to(Self::logout).wrap(WebAuthUserMiddleware),
                ),
        );
    }
}

impl SessionsController {
    async fn login(
        req: HttpRequest,
        db_conn: web::Data<SeaService>,
        body: web::Json<LoginDto>,
    ) -> AppResponse<Redirect> {
        let LoginDto { nickname, password } = match body.validate_or_back(&req) {
            Err(err) => return Ok(err),
            Ok(dto) => dto,
        };

        let service = authenticate_user_service_factory::exec(&db_conn);
        let user = match service
            .exec(AuthenticateUserParams {
                nickname: nickname.unwrap(),
                password: password.unwrap(),
            })
            .await
        {
            Err(err) => return Ok(err.into_inertia_redirect(&req)),
            Ok(user) => user,
        };

        if let Err(err) = req
            .get_session()
            .insert(SESSION_USER_KEY, user.id().to_string())
        {
            log::error!(
                "Failed to store the user id during a login request: {}",
                err
            );

            return Ok(Inertia::back_with_errors(
                &req,
                hashmap!["error" => "Não foi possível te autenticar. Tente denovo mais tarde.".into()],
            ));
        };

        Session::flash_silently(
            &req,
            "loginSuccess",
            format!("Logado com sucesso como {}!", user.nickname()),
        );

        Ok(Inertia::back(&req))
    }

    async fn register_user(
        req: HttpRequest,
        db_conn: web::Data<SeaService>,
        body: web::Json<CreateUserDto>,
    ) -> Redirect {
        let CreateUserDto { nickname, password } = match body.validate_or_back(&req) {
            Err(err_redirect) => return err_redirect,
            Ok(dto) => dto,
        };

        let create_user_service = create_user_service_factory::exec(&db_conn);
        let user = match create_user_service
            .exec(CreateUserParams { nickname, password })
            .await
        {
            Err(_) => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap![ "error" => "Alguma coisa deu errada, tente novamente mais tarde.".into() ],
                );
            }
            Ok(user) => user,
        };

        if let Err(err) = req
            .get_session()
            .insert(SESSION_USER_KEY, user.id().to_string())
        {
            log::error!(
                "Failed to store the user id during a login request: {}",
                err
            );

            return Inertia::back_with_errors(
                &req,
                hashmap!["error" => "Failed to login. Please, try again later.".into()],
            );
        };

        Session::flash_silently(
            &req,
            "registerSuccess",
            format!(
                "Registrado com sucesso! Você está logado como {}.",
                user.nickname()
            ),
        );

        Inertia::back(&req)
    }

    async fn logout(req: HttpRequest) -> Redirect {
        let _ = req.get_session().remove(SESSION_USER_KEY);
        Inertia::back(&req)
    }
}
