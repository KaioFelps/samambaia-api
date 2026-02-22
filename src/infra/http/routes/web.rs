use std::sync::Arc;

use actix_session::{SessionExt, SessionMiddleware};
use actix_web::HttpMessage;
use actix_web::body::BoxBody;
use actix_web::cookie::{Key, SameSite};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{StatusCode, header};
use actix_web::middleware::{Next, from_fn};
use actix_web::web::{self, Data};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use inertia_rust::actix::InertiaMiddleware;
use inertia_rust::{
    Inertia,
    InertiaFacade,
    InertiaProp,
    IntoInertiaError,
    IntoInertiaPropResult,
    hashmap,
    prop_resolver,
};
use serde_json::json;

use super::route::RouteTrait;
use crate::configs::app::{APP_CONFIG, SESSION_FLASH_KEY};
use crate::configs::env::RustEnv;
use crate::configs::file_sessions::FileSessionStore;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::announcements::fetch_many_announcements_service_factory;
use crate::domain::services::announcements::fetch_many_announcements_service::FetchManyAnnouncementsParams;
use crate::infra::http::controllers::admin::admin_article_tags_controller::AdminArticleTagsController;
use crate::infra::http::controllers::admin::admin_articles_controller::AdminArticlesController;
use crate::infra::http::controllers::admin::admin_home_controller::AdminHomeController;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::web::articles_controller::ArticlesController;
use crate::infra::http::controllers::web::comment_reports_controller::CommentReportsController;
use crate::infra::http::controllers::web::comments_controller::CommentsController;
use crate::infra::http::controllers::web::home_controller::HomeController;
use crate::infra::http::controllers::web::sessions_controller::SessionsController;
use crate::infra::http::middlewares::web::WebRequestUser;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::middlewares::{
    GarbageCollectorMiddleware,
    ReflashTemporarySessionMiddleware,
    WebAuthUserMiddleware,
    WebRequestUserMiddleware,
};
use crate::infra::http::presenters::announcement::AnnouncementPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::http::presenters::web_auth_user::WebAuthUserPresenter;
use crate::infra::sea::sea_service::SeaService;
use crate::util::RolePermissions;

pub struct WebRoutes;

impl RouteTrait for WebRoutes {
    fn register(cfg: &mut web::ServiceConfig) {
        let key_bytes = BASE64_STANDARD
            .decode(APP_CONFIG.app_key)
            .expect("Invalid APP_KEY value.");

        let key = Key::derive_from(key_bytes.as_slice());

        let storage = FileSessionStore::default();

        cfg.service(actix_files::Files::new("/bundle/", "./public/bundle/").prefer_utf8(true));

        cfg.service(
            web::scope("")
                .wrap(from_fn(default_error_handler))
                .wrap(GarbageCollectorMiddleware)
                .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|req| {
                    let flash = req.get_session().remove(SESSION_FLASH_KEY);

                    let flash = flash
                        .map(|map| {
                            serde_json::from_str::<serde_json::Map<_, _>>(&map).unwrap_or_default()
                        })
                        .unwrap_or_default();

                    let user = match req.extensions().get::<WebRequestUser>().cloned() {
                        None => None,
                        Some(user) => match user {
                            WebRequestUser::Ghast => None,
                            WebRequestUser::User(user) => Some(user),
                        },
                    }.map(WebAuthUserPresenter::to_http);

                    let db_conn = req
                        .app_data::<Data<SeaService>>()
                        .expect("Could not find 'SeaService' struct in the server app data.")
                        .clone();

                    Box::pin(async move {
                        hashmap![
                            "announcements" => InertiaProp::demand(prop_resolver!(let db_conn_clone = db_conn.clone(); {
                                let service = fetch_many_announcements_service_factory::exec(&db_conn_clone);
                                let announcements = service
                                    .exec(FetchManyAnnouncementsParams {
                                        page: None,
                                        per_page: None,
                                        query: None,
                                    })
                                    .await
                                    .map_err(IntoInertiaError::into_inertia_error)?;

                                AnnouncementPresenter::to_json_paginated_wrapper(
                                    announcements.data,
                                    announcements.pagination,
                                    DEFAULT_PER_PAGE,
                                ).into_inertia_value()
                            })),
                            "auth" => InertiaProp::always(user),
                            "flash" => InertiaProp::always(flash),
                            "verificationMotto" => InertiaProp::data(APP_CONFIG.verification_motto),
                            // TODO: adicionar o domínio de membros destaques
                            "featuredUsers" => InertiaProp::data(json!({
                                "data": [],
                                "pagination": {
                                    "currentPage": 0,
                                    "totalItems": 0,
                                    "totalPages": 0,
                                    "itemsPerPage": 0,
                                }
                            }))
                        ]
                    })
                })))
                .wrap(WebRequestUserMiddleware)
                .wrap(ReflashTemporarySessionMiddleware)
                .wrap(
                    SessionMiddleware::builder(storage, key)
                        .cookie_domain(None)
                        .cookie_http_only(true)
                        .cookie_same_site(SameSite::Lax)
                        .cookie_name(APP_CONFIG.session_cookie.into())
                        .cookie_secure(APP_CONFIG.rust_env == RustEnv::Production)
                        .build(),
                )
                .configure(HomeController::register)
                .configure(SessionsController::register)
                .configure(ArticlesController::register)
                .configure(CommentsController::register)
                .configure(CommentReportsController::register)
                .configure(|cfg| {
                    cfg.service(web::scope("/gremio")
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::AccessDashboard],
                            PermissionComparisonMode::All,
                        ))
                        .wrap(WebAuthUserMiddleware)
                        .configure(AdminArticleTagsController::register)
                        .configure(AdminArticlesController::register)
                        // needs to be the last, since captures everything else by having "" in its scope.
                        .configure(AdminHomeController::register)
                    );
                })
                .service(actix_files::Files::new("/", "./public/").prefer_utf8(true))
            );
    }
}

async fn default_error_handler(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::error::Error> {
    let is_inertia_request = Inertia::check_is_inertia_request(req.request());

    let accepts_views_as_response = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.contains("text/html"))
        .unwrap_or(false);

    let res = next.call(req).await?;

    let can_send_error_view = is_inertia_request || accepts_views_as_response;

    if !can_send_error_view {
        return Ok(res);
    }

    let status = res.status().as_u16();

    if [503, 500, 404, 403, 401].contains(&status) {
        log::debug!("Request has fallen to default error handler.");

        if APP_CONFIG.rust_env != RustEnv::Production {
            log::debug!("{:#?}", res.response().body());
        }

        let mut inertia_err_response = Inertia::render_with_props(
            res.request(),
            "error".into(),
            hashmap![ "status" => InertiaProp::data(status) ],
        )
        .await?;

        *inertia_err_response.status_mut() = StatusCode::from_u16(status).unwrap();

        return Ok(res.into_response(inertia_err_response));
    }

    Ok(res)
}
