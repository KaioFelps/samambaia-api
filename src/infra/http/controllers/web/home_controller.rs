use actix_web::web::{self, Data};
use actix_web::HttpRequest;
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};

use crate::domain::factories::journalism::articles::fetch_home_page_articles_service_factory;
use crate::domain::factories::journalism::free_badges::fetch_many_free_badges_service_factory;
use crate::domain::services::journalism::free_badges::fetch_many_free_badges_service::{
    FetchManyFreeBadgesParams,
    FetchManyFreeBadgesResponse,
};
use crate::error::IntoSamambaiaError;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::dtos::controllers::home::HomeQueryDto;
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::infra::http::presenters::free_badge::FreeBadgePresenter;
use crate::infra::http::presenters::home_article::MappedHomeArticle;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;

pub struct HomeController;

impl ControllerTrait for HomeController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.route("/", web::get().to(Self::home));
        cfg.route("foo", web::get().to(Self::foo).wrap(WebAuthUserMiddleware));
    }
}

impl HomeController {
    async fn foo(req: HttpRequest) -> AppResponse {
        Inertia::render(&req, "foo".into())
            .await
            .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    async fn home(
        req: HttpRequest,
        db_conn: Data<SeaService>,
        query: web::Query<HomeQueryDto>,
    ) -> AppResponse {
        const FREE_BADGES_PER_PAGE: u8 = 26;

        let articles_service = fetch_home_page_articles_service_factory::exec(&db_conn);
        let free_badges_service = fetch_many_free_badges_service_factory::exec(&db_conn);

        let articles_future = articles_service.exec();
        let free_badge_future = free_badges_service.exec(FetchManyFreeBadgesParams {
            per_page: Some(FREE_BADGES_PER_PAGE as u32),
            page: query.free_badges_page,
        });

        let (articles, free_badge): (Vec<MappedHomeArticle>, FetchManyFreeBadgesResponse) =
            tokio::try_join!(articles_future, free_badge_future)?;

        let free_badge = FreeBadgePresenter::to_json_paginated_wrapper(
            free_badge.data,
            free_badge.pagination,
            FREE_BADGES_PER_PAGE,
        );

        Inertia::render_with_props(
            &req,
            "index".into(),
            hashmap![
                "articles" => InertiaProp::data(articles),
                "freeBadges" => InertiaProp::data(free_badge)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }
}
