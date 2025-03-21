use actix_web::web::{self, Data, Query};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::user::User;
use crate::domain::factories::journalism::articles::fetch_many_articles_service_factory;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::domain::services::journalism::articles::fetch_many_articles_service::{
    FetchManyArticlesParams,
    FetchManyArticlesService,
    ServiceArticleQueryType,
};
use crate::error::{IntoSamambaiaError, SamambaiaError};
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::AppResponseRedirect;
use crate::infra::http::dtos::list_article_admin::AdminListArticlesDto;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::presenters::article::{ArticlePresenter, MappedArticle};
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedPaginatedEntity, PresenterTrait};
use crate::infra::sea::sea_service::SeaService;
use crate::util::{verify_role_has_permission, RolePermissions};

pub struct AdminArticlesController;

impl ControllerTrait for AdminArticlesController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/noticias").route(
                "",
                web::get()
                    .wrap(WebHasPermissionMiddleware::new(
                        vec![
                            RolePermissions::SeeUnapprovedArticle,
                            RolePermissions::CreateArticle,
                        ],
                        PermissionComparisonMode::Any,
                    ))
                    .to(Self::manage_articles),
            ),
        );
    }
}

impl AdminArticlesController {
    async fn manage_articles(
        req: HttpRequest,
        db_conn: Data<SeaService>,
        query: Query<AdminListArticlesDto>,
        auth: WebAuthUser,
    ) -> AppResponseRedirect {
        let query = match query
            .into_inner()
            .validate_or_back(&req)
            .map_err(Either::Right)
        {
            Ok(query) => query,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let find_articles_service = fetch_many_articles_service_factory::exec(&db_conn);

        let articles = Self::get_mapped_articles(find_articles_service, &auth.user, query).await?;

        Inertia::render_with_props(
            &req,
            "admin/articles/index".into(),
            hashmap![
                "articles" => InertiaProp::data(articles)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
        .map(Either::Left)
    }
}

impl AdminArticlesController {
    async fn get_mapped_articles<AR: ArticleRepositoryTrait, UR: UserRepositoryTrait>(
        service: FetchManyArticlesService<AR, UR>,
        user: &User,
        query: AdminListArticlesDto,
    ) -> Result<JsonWrappedPaginatedEntity<MappedArticle>, SamambaiaError> {
        let user_can_see_unnaproved_articles = verify_role_has_permission(
            user.role().as_ref().unwrap(),
            RolePermissions::SeeUnapprovedArticle,
        );

        service
            .exec(FetchManyArticlesParams {
                page: query.page,
                per_page: query.per_page.map(|pp| pp as u32),
                approved_state: if user_can_see_unnaproved_articles {
                    query.approved_state
                } else {
                    Some(false)
                },
                query: if let Some(author) = query.author {
                    Some(ServiceArticleQueryType::Author(author))
                } else {
                    query.title.map(ServiceArticleQueryType::Title)
                },
            })
            .await
            .map(|articles| {
                let mapped_articles = articles
                    .data
                    .into_iter()
                    .map(ArticlePresenter::to_http)
                    .collect::<Vec<_>>();

                let mapped_pagination = PaginationPresenter::to_http(
                    articles.pagination,
                    query.per_page.unwrap_or(DEFAULT_PER_PAGE),
                );

                JsonWrappedPaginatedEntity {
                    data: mapped_articles,
                    pagination: mapped_pagination,
                }
            })
    }
}
