use actix_web::HttpRequest;
use actix_web::web::{self, Data, Path, Query};
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use serde_qs::web::QsQuery;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::journalism::article_tags::fetch_many_article_tags_service_factory;
use crate::domain::factories::journalism::articles::{
    fetch_articles_previews_service_factory,
    get_expanded_article_service_factory,
};
use crate::domain::services::journalism::article_tags::fetch_many_article_tags_service::FetchManyArticleTagsParams;
use crate::domain::services::journalism::articles::fetch_articles_services::{
    FetchArticleQuery,
    FetchArticlesParams,
};
use crate::domain::services::journalism::articles::get_expanded_article_service::GetExpandedArticleParams;
use crate::domain::value_objects::slug::Slug;
use crate::error::IntoSamambaiaError;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::dtos::controllers::articles::{
    ArticlesArchiveQueryDto,
    ShowArticleQueryDto,
};
use crate::infra::http::middlewares::web::WebRequestUser;
use crate::infra::http::presenters::article_preview::ArticlePreviewPresenter;
use crate::infra::http::presenters::article_tag::ArticleTagPresenter;
use crate::infra::http::presenters::expanded_article::ExpandedArticlePresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;
use crate::libs::validator_extensions::FilterQueryExt;

pub struct ArticlesController;

impl ControllerTrait for ArticlesController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/noticias")
                // CREATE
                .route("/arquivo", web::get().to(Self::archive))
                .route("/{slug}", web::get().to(Self::show)),
        );
    }
}

impl ArticlesController {
    pub async fn show(
        slug: Path<String>,
        db_conn: Data<SeaService>,
        auth_user: WebRequestUser,
        query: Query<ShowArticleQueryDto>,
        req: HttpRequest,
    ) -> AppResponse {
        let auth_user = match auth_user {
            WebRequestUser::Ghast => None,
            WebRequestUser::User(user) => Some(user),
        };

        let get_expanded_article_service = get_expanded_article_service_factory::exec(&db_conn);
        let fetch_articles_service = fetch_articles_previews_service_factory::exec(&db_conn);

        const ARTICLES_AMOUNT_TO_DISPLAY_IN_FEED: u8 = 16;
        let user_id = auth_user.as_ref().map(|auth| auth.user.id());
        let user_role = auth_user.as_ref().map(|auth| auth.user.role().unwrap());
        let (expanded_article, last_16_articles) = tokio::try_join!(
            get_expanded_article_service.exec(GetExpandedArticleParams {
                user_id: user_id.as_ref(),
                user_role: user_role.as_ref(),
                article_slug: Slug::new_from_existing(slug.into_inner()),
                comments_per_page: None,
                comments_page: query.comments_page,
            }),
            fetch_articles_service.exec(FetchArticlesParams {
                approved_state: Some(true),
                page: Some(1),
                per_page: Some(ARTICLES_AMOUNT_TO_DISPLAY_IN_FEED as u32),
                queries: vec![],
            })
        )?;

        let expanded_article = ExpandedArticlePresenter::to_http(
            expanded_article.article,
            expanded_article.article_author,
            expanded_article.comments.data,
            (expanded_article.comments.pagination, DEFAULT_PER_PAGE),
        );

        let last_16_articles = ArticlePreviewPresenter::to_json_paginated_wrapper(
            last_16_articles.data,
            last_16_articles.pagination,
            ARTICLES_AMOUNT_TO_DISPLAY_IN_FEED,
        );

        Inertia::render_with_props(
            &req,
            "articles/show".into(),
            hashmap![
                "article" => InertiaProp::data(expanded_article),
                "feed" => InertiaProp::data(last_16_articles)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    pub async fn archive(
        db_conn: Data<SeaService>,
        query: QsQuery<ArticlesArchiveQueryDto>,
        req: HttpRequest,
    ) -> AppResponse {
        const PER_PAGE: u8 = 10;

        let fetch_articles_service = fetch_articles_previews_service_factory::exec(&db_conn);
        let list_tags_service = fetch_many_article_tags_service_factory::exec(&db_conn);

        let (query, query_errors) = query.into_inner().validate_and_sanitize();

        dbg!(&query);
        let service_queries = vec![
            query.title.map(FetchArticleQuery::Title),
            query.author.map(FetchArticleQuery::Author),
            query.including_tags.map(FetchArticleQuery::Tags),
        ]
        .into_iter()
        .flatten()
        .collect();

        dbg!(&service_queries);

        let (article_tags, articles) = tokio::try_join!(
            list_tags_service.exec(FetchManyArticleTagsParams {
                page: None,
                per_page: Some(100),
                query: None,
            }),
            fetch_articles_service.exec(FetchArticlesParams {
                approved_state: Some(true),
                page: query.page.map(|page| page as u32),
                per_page: Some(PER_PAGE as u32),
                queries: service_queries,
            }),
        )?;

        let article_tags = article_tags
            .data
            .into_iter()
            .map(ArticleTagPresenter::to_http)
            .collect::<Vec<_>>();

        let articles = ArticlePreviewPresenter::to_json_paginated_wrapper(
            articles.data,
            articles.pagination,
            PER_PAGE,
        );

        Inertia::render_with_props(
            &req,
            "articles/archive".into(),
            hashmap![
                "tags" => InertiaProp::data(article_tags),
                "feed" => InertiaProp::data(articles),
                "queryErrors" => InertiaProp::data(query_errors)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }
}
