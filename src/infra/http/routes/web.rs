use super::route::RouteTrait;
use crate::configs::app::{APP_CONFIG, SESSION_FLASH_KEY, SESSION_USER_KEY};
use crate::configs::env::RustEnv;
use crate::configs::file_sessions::FileSessionStore;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::announcements::fetch_many_announcements_service_factory;
use crate::domain::services::announcements::fetch_many_announcements_service::FetchManyAnnouncementsParams;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::web::home_controller::HomeController;
use crate::infra::http::controllers::web::sessions_controller::SessionsController;
use crate::infra::http::middlewares::web::WebRequestUser;
use crate::infra::http::middlewares::{
    GarbageCollectorMiddleware, ReflashTemporarySessionMiddleware, WebAuthUserMiddleware,
    WebRequestUserMiddleware,
};
use crate::infra::http::presenters::announcement::AnnouncementPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;
use actix_session::{SessionExt, SessionMiddleware};
use actix_web::body::BoxBody;
use actix_web::cookie::{Key, SameSite};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::StatusCode;
use actix_web::middleware::{from_fn, Next};
use actix_web::web::{self, Data};
use actix_web::HttpMessage;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use inertia_rust::actix::InertiaMiddleware;
use inertia_rust::{
    hashmap, prop_resolver, Inertia, InertiaFacade, InertiaProp, IntoInertiaError,
    IntoInertiaPropResult,
};
use serde_json::json;
use std::sync::Arc;

pub struct WebRoutes;

impl RouteTrait for WebRoutes {
    fn register(cfg: &mut web::ServiceConfig) {
        let key_bytes = BASE64_STANDARD
            .decode(APP_CONFIG.app_key)
            .expect("Invalid APP_KEY value.");

        let key = Key::derive_from(key_bytes.as_slice());

        let storage = FileSessionStore::default();

        cfg.service(
            web::scope("")
                .wrap(GarbageCollectorMiddleware)
                .wrap(from_fn(default_error_handler))
                .wrap(WebAuthUserMiddleware)
                .wrap(WebRequestUserMiddleware)
                .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|req| {
                    let flash = req
                        .get_session()
                        .remove(SESSION_FLASH_KEY);

                    let flash= flash.map(|map| serde_json::from_str::<serde_json::Map<_, _>>(&map).unwrap_or_default())
                        .unwrap_or_default();


                    // let req = req.clone();

                    let user = if let Some(WebRequestUser::User(user)) = req.extensions()
                        .get::<WebRequestUser>()
                        .cloned() { Some(user) } else { None };

                    let user = InertiaProp::always(user);

                    let db_conn = req
                        .app_data::<Data<SeaService>>()
                        .expect("Could not find 'SeaService' struct in the server app data.")
                        .clone();

                    Box::pin(async move {
                        hashmap![
                            "announcements" => InertiaProp::lazy(prop_resolver!(let db_conn_clone = db_conn.clone(); {
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
                            "auth" => user,
                            "flash" => InertiaProp::always(flash),
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
                .wrap(ReflashTemporarySessionMiddleware)
                .wrap(SessionMiddleware::builder(storage, key)
                    .cookie_domain(Some(APP_CONFIG.domain.into()))
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .cookie_name(APP_CONFIG.session_cookie.into())
                    .cookie_secure(APP_CONFIG.rust_env == RustEnv::Production)
                    .build())
                .configure(HomeController::register)
                .configure(SessionsController::register)
                .configure(|cfg| {
                    // serves public assets directly from /path
                    // needs to be the last service because it's a wildcard
                    cfg.service(actix_files::Files::new("/", "./public/").prefer_utf8(true));
                }),
        );
    }
}

async fn default_error_handler(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::error::Error> {
    let res = next.call(req).await?;
    let status = res.status().as_u16();

    if APP_CONFIG.rust_env != RustEnv::Development && [503, 500, 404, 403].contains(&status) {
        let mut inertia_err_response = Inertia::render_with_props(
            res.request(),
            "Error".into(),
            hashmap![ "status" => InertiaProp::data(status) ],
        )
        .await?;

        *inertia_err_response.status_mut() = StatusCode::from_u16(status).unwrap();

        return Ok(res.into_response(inertia_err_response));
    }

    Ok(res)
}
