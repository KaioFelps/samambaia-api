use actix_web::{
    web::{self, Data},
    HttpRequest,
};
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};

use crate::{
    domain::factories::journalism::articles::fetch_home_page_articles_service_factory,
    error::IntoDomainError,
    infra::{
        http::controllers::{controller::ControllerTrait, AppResponse},
        sea::sea_service::SeaService,
    },
};

pub struct HomeController;

impl ControllerTrait for HomeController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.route("/", web::get().to(Self::home));
    }
}

impl HomeController {
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
        .map_err(IntoDomainError::into_domain_error)
    }
}
