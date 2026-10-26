use actix_session::{Session, SessionExt};
use actix_web::HttpRequest;
use actix_web::web::{self, Redirect, ThinData};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use jobs::logging::{ActionKind, Actor, EntitySnapshot, LoggingJobBuilder, Target, TargetType};
use jobs::queue::JobQueue;

use crate::configs::app::{APP_CONFIG, SESSION_USER_KEY};
use crate::configs::inertia::IntoInertiaRedirect;
use crate::domain::domain_entities::user::User;
use crate::domain::factories::identity::{
    authenticate_user_service_factory,
    create_user_service_factory,
    verify_verification_code_service_factory,
};
use crate::domain::services::identity::authenticate_user_service::AuthenticateUserParams;
use crate::domain::services::identity::create_user_service::CreateUserParams;
use crate::domain::services::identity::verify_verification_code_service::VerifyVerificationCodeParams;
use crate::error::SamambaiaError::{Internal, UserAlreadyExists};
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::dtos::create_user::CreateUserDto;
use crate::infra::http::dtos::login::LoginDto;
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::http::presenters::user::UserPresenter;
use crate::infra::sea::sea_service::SeaService;

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
        job_queue: ThinData<JobQueue>,
    ) -> Redirect {
        let body = match body.validate_or_back(&req) {
            Err(err_redirect) => return err_redirect,
            Ok(dto) => dto,
        };

        let nickname = body.nickname.unwrap();
        let password = body.password.unwrap();

        let code_verification_service = verify_verification_code_service_factory::exec();
        let code_authorization = match code_verification_service
            .execute(VerifyVerificationCodeParams {
                nickname: &nickname,
            })
            .await
        {
            Ok(authorization) => authorization,
            Err(_) => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap![ "error" => "Não conseguimos nos comunicar com o hotel, tente novamente mais tarde.".into() ],
                );
            }
        };

        if !code_authorization.is_authorized {
            return Inertia::back_with_errors(
                &req,
                hashmap![
                    "verification_code" => format!(
                        "Mude sua missão para {} para se registrar (atual: \"{}\").",
                        APP_CONFIG.verification_motto,
                        code_authorization.motto.unwrap_or_default()
                    ).into()
                ],
            );
        }

        let create_user_service = create_user_service_factory::exec(&db_conn);
        let user = match create_user_service
            .exec(CreateUserParams {
                nickname: nickname.clone(),
                password,
            })
            .await
        {
            Ok(user) => user,
            Err(err) => {
                let error = if let UserAlreadyExists(err_msg) = err {
                    err_msg
                } else {
                    if let Internal(err) = err {
                        log::error!("Failed to register the user {nickname}: {err}",);
                    }

                    "Alguma coisa deu errada, tente novamente mais tarde.".to_string()
                };

                return Inertia::back_with_errors(&req, hashmap![ "error" => error.into() ]);
            }
        };

        if let Err(err) = req
            .get_session()
            .insert(SESSION_USER_KEY, user.id().to_string())
        {
            log::error!(
                "Failed to store the user id during a user account register request: {}",
                err
            );

            return Inertia::back_with_errors(
                &req,
                hashmap!["error" => "Não foi possível registrar a sua conta. \
                Por favor, tente novamente mais tarde.".into()],
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

        enqueue_session_audit_log(&job_queue, ActionKind::Create, &user).await;

        Inertia::back(&req)
    }

    async fn logout(req: HttpRequest) -> Redirect {
        let _ = req.get_session().remove(SESSION_USER_KEY);
        Inertia::back(&req)
    }
}

async fn enqueue_session_audit_log(job_queue: &JobQueue, action: ActionKind, user: &User) {
    let _ = action;
    let _ = job_queue;
    let Ok(user_snapshot) = EntitySnapshot::new(
        user.id().to_string(),
        user.nickname().to_string(),
        UserPresenter::to_http(user.clone()),
    ) else {
        log::error!("Failed to create User snapshot for audit log");
        return;
    };

    let builder = LoggingJobBuilder::default()
        .action_kind(action.clone())
        .actor(Actor::System)
        .target(Target {
            t_type: TargetType::User,
            entity: user_snapshot,
        });

    match builder.build() {
        Ok(log_data) => {
            if let Err(err) = job_queue.enqueue(log_data.into_job()).await {
                log::error!(
                    "Error enqueueing audit log ({:?}) for User: {err:?}",
                    action
                );
            }
        }
        Err(err) => {
            log::error!("Error building audit log ({:?}) for User: {err}", action);
        }
    }
}
