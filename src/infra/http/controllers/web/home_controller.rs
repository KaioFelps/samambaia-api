use actix_web::{
    web::{self, Data},
    HttpRequest,
};
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};

use crate::{
    domain::factories::journalism::articles::fetch_home_page_articles_service_factory,
    error::IntoSamambaiaError,
    infra::{
        http::{
            controllers::{controller::ControllerTrait, AppResponse},
            middlewares::WebAuthUserMiddleware,
        },
        sea::sea_service::SeaService,
    },
};

pub struct HomeController;

impl ControllerTrait for HomeController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.route("/", web::get().to(Self::home));
        cfg.route("foo", web::get().to(Self::foo).wrap(WebAuthUserMiddleware));
    }
}

impl HomeController {
    async fn foo(req: HttpRequest) -> AppResponse {
        Inertia::render(&req, "Foo".into())
            .await
            .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    async fn home(req: HttpRequest, db_conn: Data<SeaService>) -> AppResponse {
        let articles_service = fetch_home_page_articles_service_factory::exec(&db_conn);
        let articles = articles_service.exec().await?;

        Inertia::render_with_props(
            &req,
            "Index".into(),
            hashmap![
                "articles" => InertiaProp::data(articles)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }
}
