use std::path::Path;
use std::sync::Arc;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::announcements::fetch_many_announcements_service_factory;
use crate::domain::services::announcements::fetch_many_announcements_service::FetchManyAnnouncementsParams;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::web::home_controller::HomeController;
use crate::infra::http::middlewares::RequestUserMiddleware;
use crate::infra::http::presenters::announcement::AnnouncementPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;
use actix_web::web::{self, Data};
use inertia_rust::actix::InertiaMiddleware;
use inertia_rust::{hashmap, prop_resolver, InertiaProp};
use serde_json::to_value;

use super::route::RouteTrait;

pub struct WebRoutes;

impl RouteTrait for WebRoutes {
    fn register(cfg: &mut web::ServiceConfig) {
        // adds this service only if the directory exists
        if Path::new("public/bundle/assets").exists() {
            // serves vite assets from /assets path
            cfg.service(
                actix_files::Files::new("/assets", "./public/bundle/assets").prefer_utf8(true),
            );
        }

        cfg.service(
            web::scope("")
                .wrap(RequestUserMiddleware)
                .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|req| {
                    let req = req.clone();
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
                                    .unwrap();

                                to_value(AnnouncementPresenter::to_json_paginated_wrapper(
                                    announcements.data,
                                    announcements.pagination,
                                    DEFAULT_PER_PAGE,
                                ))
                                .unwrap()
                            }))
                        ]
                    })
                })))
                .configure(HomeController::register)
                .configure(|cfg| {
                    // serves public assets directly from /path
                    // needs to be the last service because it's a wildcard
                    cfg.service(actix_files::Files::new("/", "./public/").prefer_utf8(true));
                }),
        );
    }
}
