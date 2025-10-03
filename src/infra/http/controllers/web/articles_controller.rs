use actix_web::web::{self, Data, Path, Query};
use actix_web::HttpRequest;
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::journalism::articles::get_expanded_article_service_factory;
use crate::domain::services::journalism::articles::get_expanded_article_service::GetExpandedArticleParams;
use crate::domain::value_objects::slug::Slug;
use crate::error::IntoSamambaiaError;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::dtos::controllers::articles::ShowArticleQueryDto;
use crate::infra::http::middlewares::web::WebRequestUser;
use crate::infra::http::presenters::expanded_article::ExpandedArticlePresenter;
use crate::infra::sea::sea_service::SeaService;

pub struct ArticlesController;

impl ControllerTrait for ArticlesController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/noticias")
                // CREATE
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

        let service = get_expanded_article_service_factory::exec(&db_conn);

        let expanded_article = service
            .exec(GetExpandedArticleParams {
                user_id: auth_user
                    .as_ref()
                    .map(|auth_user| auth_user.user.id())
                    .as_ref(),
                user_role: auth_user
                    .as_ref()
                    .map(|auth_user| auth_user.user.role().unwrap())
                    .as_ref(),
                article_slug: Slug::new_from_existing(slug.into_inner()),
                comments_per_page: None,
                comments_page: query.comments_page,
            })
            .await?;

        let expanded_article = ExpandedArticlePresenter::to_http(
            expanded_article.article,
            expanded_article.article_author,
            expanded_article.comments.data,
            (expanded_article.comments.pagination, DEFAULT_PER_PAGE),
        );

        Inertia::render_with_props(
            &req,
            "articles/index".into(),
            hashmap![
                "article" => InertiaProp::data(expanded_article)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }
}
