use actix_session::Session;
use actix_web::web::{self, Data, Json, Query, Redirect};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::journalism::article_tags::{
    create_article_tag_service_factory,
    fetch_many_article_tags_service_factory,
};
use crate::domain::services::journalism::article_tags::create_article_tag_service::CreateArticleTagParams;
use crate::domain::services::journalism::article_tags::fetch_many_article_tags_service::FetchManyArticleTagsParams;
use crate::error::IntoSamambaiaError;
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::{AppResponse, AppResponseRedirect};
use crate::infra::http::dtos::create_article_tag::CreateArticleTagDto;
use crate::infra::http::dtos::list_article_tags::ListArticleTagsDto;
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::presenters::article_tag::ArticleTagPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;
use crate::util::RolePermissions;

pub struct AdminArticleTagsController;

impl ControllerTrait for AdminArticleTagsController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/tags")
                .route(
                    "",
                    web::get()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticle],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::manage),
                )
                .route(
                    "nova",
                    web::get()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticleTag],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::create),
                )
                .route(
                    "criar",
                    web::post()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticleTag],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::store),
                ),
        );
    }
}

impl AdminArticleTagsController {
    async fn manage(
        req: HttpRequest,
        db_conn: Data<SeaService>,
        query: Query<ListArticleTagsDto>,
    ) -> AppResponseRedirect {
        let query = match query
            .into_inner()
            .validate_or_back(&req)
            .map_err(Either::Right)
        {
            Ok(query) => query,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let find_articles_service = fetch_many_article_tags_service_factory::exec(&db_conn);

        let per_page = query.per_page.unwrap_or(DEFAULT_PER_PAGE);

        let tags = find_articles_service
            .exec(FetchManyArticleTagsParams {
                page: query.page,
                per_page: Some(per_page as u32),
                query: query.value,
            })
            .await?;

        let tags =
            ArticleTagPresenter::to_json_paginated_wrapper(tags.data, tags.pagination, per_page);

        Inertia::render_with_props(
            &req,
            "admin/articles/tags/index".into(),
            hashmap![
                "tags" => InertiaProp::data(tags)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
        .map(Either::Left)
    }

    async fn create(req: HttpRequest) -> AppResponse {
        Inertia::render(&req, "admin/articles/tags/new".into())
            .await
            .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    async fn store(
        req: HttpRequest,
        body: Json<CreateArticleTagDto>,
        db_conn: Data<SeaService>,
        auth_user: WebAuthUser,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Err(redirect) => return Ok(redirect),
            Ok(body) => body,
        };

        let create_article_tag_service = create_article_tag_service_factory::exec(&db_conn);

        let tag = create_article_tag_service
            .exec(CreateArticleTagParams {
                value: body.value,
                user_role: auth_user.user.role().unwrap(),
            })
            .await?;

        Session::flash_silently(
            &req,
            "createArticleTagSuccess",
            format!(
                "Tag {} criada com sucesso com ID {}!",
                tag.value(),
                tag.id()
            ),
        );

        Ok(Inertia::back(&req))
    }
}
