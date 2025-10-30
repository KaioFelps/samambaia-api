use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{self, Data, Json, Path, Query, Redirect};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{hashmap, Inertia, InertiaFacade, InertiaProp};
use uuid::Uuid;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::user::User;
use crate::domain::factories::journalism::article_tags::fetch_many_article_tags_service_factory;
use crate::domain::factories::journalism::articles::{
    create_article_service_factory,
    delete_article_service_factory,
    fetch_articles_previews_service_factory,
    find_article_by_id_service_factory,
    update_article_service_factory,
};
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::domain::services::journalism::article_tags::fetch_many_article_tags_service::FetchManyArticleTagsParams;
use crate::domain::services::journalism::articles::create_article_service::CreateArticleParams;
use crate::domain::services::journalism::articles::delete_article_service::DeleteArticleParams;
use crate::domain::services::journalism::articles::fetch_articles_previews_service::FetchArticlesPreviewsService;
use crate::domain::services::journalism::articles::fetch_articles_services::{
    FetchArticleQuery,
    FetchArticlesParams,
};
use crate::domain::services::journalism::articles::find_article_by_id_service::FindArticleByIdParams;
use crate::domain::services::journalism::articles::update_article_service::UpdateArticleParams;
use crate::error::{IntoSamambaiaError, SamambaiaError};
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::{AppResponse, AppResponseRedirect};
use crate::infra::http::dtos::create_article::CreateArticleDto;
use crate::infra::http::dtos::list_article_admin::AdminListArticlesDto;
use crate::infra::http::dtos::patch_article_approved::PatchArticleApprovedDto;
use crate::infra::http::dtos::update_article::UpdateArticleDto;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::presenters::article::ArticlePresenter;
use crate::infra::http::presenters::article_preview::{
    ArticlePreviewPresenter,
    MappedArticlePreview,
};
use crate::infra::http::presenters::article_tag::ArticleTagPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedPaginatedEntity, PresenterTrait};
use crate::infra::sea::sea_service::SeaService;
use crate::util::{verify_role_has_permission, RolePermissions};

pub struct AdminArticlesController;

impl ControllerTrait for AdminArticlesController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/noticias")
                .route(
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
                )
                .route(
                    "nova",
                    web::get()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticle],
                            PermissionComparisonMode::All,
                        ))
                        .to(Self::create_article),
                )
                .route(
                    "criar",
                    web::post()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticle],
                            PermissionComparisonMode::All,
                        ))
                        .to(Self::store_article),
                )
                .route(
                    "{article_id}/alterar-aprovado",
                    web::patch()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::ApproveArticle],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::toggle_article_approved),
                )
                .route(
                    "{article_id}/editar",
                    web::get()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticle],
                            PermissionComparisonMode::All,
                        ))
                        .to(Self::edit_article),
                )
                .route(
                    "{article_id}/atualizar",
                    web::put()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticle],
                            PermissionComparisonMode::All,
                        ))
                        .to(Self::update_article),
                )
                .route(
                    "{article_id}/apagar",
                    web::delete()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![
                                RolePermissions::DeleteArticle,
                                RolePermissions::DeleteComment,
                            ],
                            PermissionComparisonMode::All,
                        ))
                        .to(Self::delete_article),
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

        let find_articles_service = fetch_articles_previews_service_factory::exec(&db_conn);

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

    async fn create_article(req: HttpRequest, db_conn: Data<SeaService>) -> AppResponse {
        let list_tags_service = fetch_many_article_tags_service_factory::exec(&db_conn);
        let article_tags = list_tags_service
            .exec(FetchManyArticleTagsParams {
                page: None,
                per_page: Some(100),
                query: None,
            })
            .await?;

        let article_tags = article_tags
            .data
            .into_iter()
            .map(ArticleTagPresenter::to_http)
            .collect::<Vec<_>>();

        Inertia::render_with_props(
            &req,
            "admin/articles/new".into(),
            hashmap![
                "tags" => InertiaProp::data(article_tags)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    async fn edit_article(
        req: HttpRequest,
        path: Path<Uuid>,
        auth: WebAuthUser,
        db_conn: Data<SeaService>,
    ) -> AppResponse {
        let find_article_service = find_article_by_id_service_factory::exec(&db_conn);
        let article_tags_service = fetch_many_article_tags_service_factory::exec(&db_conn);

        let (article, tags) = tokio::try_join!(
            find_article_service.exec(FindArticleByIdParams {
                article_id: path.into_inner(),
                user: Some(&auth.user),
            }),
            article_tags_service.exec(FetchManyArticleTagsParams {
                page: None,
                per_page: Some(100),
                query: None
            })
        )?;

        let article = article.map(ArticlePresenter::to_http);
        let tags = tags
            .data
            .into_iter()
            .map(ArticleTagPresenter::to_http)
            .collect::<Vec<_>>();

        Inertia::render_with_props(
            &req,
            "admin/articles/edit".into(),
            hashmap![
                "article" => InertiaProp::data(article),
                "tags" => InertiaProp::data(tags)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    async fn store_article(
        req: HttpRequest,
        db_conn: Data<SeaService>,
        body: Json<CreateArticleDto>,
        auth: WebAuthUser,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let service = create_article_service_factory::exec(&db_conn);
        let _article = service
            .exec(CreateArticleParams {
                title: body.title,
                content: body.content,
                cover_url: body.cover_url,
                custom_author_id: body.author_id,
                description: body.description,
                staff: &auth.user,
                tags: body.tags,
                script: None,
            })
            .await?;

        Session::flash_silently(&req, "createArticleSuccess", "Notícia criada com sucesso!");

        Ok(Redirect::to("/gremio/noticias").using_status_code(StatusCode::FOUND))
        // Ok(Inertia::back(&req))
    }

    async fn toggle_article_approved(
        req: HttpRequest,
        db_conn: Data<SeaService>,
        body: Json<PatchArticleApprovedDto>,
        auth: WebAuthUser,
        article_id: Path<Uuid>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let article_id = article_id.into_inner();

        let service = update_article_service_factory::exec(&db_conn);
        if let Err(err) = service
            .exec(UpdateArticleParams {
                user: &auth.user,
                article_id,
                approved: body.approved,
                author_id: None,
                content: None,
                cover_url: None,
                description: None,
                title: None,
                tags: None,
            })
            .await
        {
            return Ok(Inertia::back_with_errors(
                &req,
                hashmap![
                   "error" => err.get_message().to_string().into()
                ],
            ));
        }

        Ok(Inertia::back(&req))
    }

    async fn update_article(
        req: HttpRequest,
        auth: WebAuthUser,
        path: Path<Uuid>,
        body: Json<UpdateArticleDto>,
        db_conn: Data<SeaService>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let service = update_article_service_factory::exec(&db_conn);

        match service
            .exec(UpdateArticleParams {
                title: body.title,
                description: body.description,
                content: body.content,
                approved: body.approved,
                cover_url: body.cover_url,
                article_id: path.into_inner(),
                author_id: body.author_id,
                user: &auth.user,
                tags: body.tags,
            })
            .await
        {
            Err(err) => Ok(Inertia::back_with_errors(
                &req,
                hashmap![
                    "error" => err.get_message().to_string().into()
                ],
            )),
            Ok(_article) => {
                Session::flash_silently(
                    &req,
                    "editArticleSuccess",
                    "Notícia atualizada com sucesso!",
                );
                Ok(Inertia::back(&req))
            }
        }
    }

    async fn delete_article(
        req: HttpRequest,
        auth: WebAuthUser,
        path: Path<Uuid>,
        db_conn: Data<SeaService>,
    ) -> AppResponse<Redirect> {
        let service = delete_article_service_factory::exec(&db_conn);

        if let Err(err) = service
            .exec(DeleteArticleParams {
                article_id: path.into_inner(),
                user: &auth.user,
            })
            .await
        {
            return Ok(Inertia::back_with_errors(
                &req,
                hashmap![
                    "error" => err.get_message().to_string().into()
                ],
            ));
        }

        Session::flash_silently(&req, "deleteArticleSuccess", "Notícia apagada com sucesso!");
        Ok(Inertia::back(&req))
    }
}

impl AdminArticlesController {
    async fn get_mapped_articles<AR: ArticleRepositoryTrait, UR: UserRepositoryTrait>(
        service: FetchArticlesPreviewsService<AR, UR>,
        user: &User,
        query: AdminListArticlesDto,
    ) -> Result<JsonWrappedPaginatedEntity<MappedArticlePreview>, SamambaiaError> {
        let user_can_see_unnaproved_articles = verify_role_has_permission(
            user.role().as_ref().unwrap(),
            RolePermissions::SeeUnapprovedArticle,
        );

        let per_page = query.per_page.unwrap_or(DEFAULT_PER_PAGE);

        service
            .exec(FetchArticlesParams {
                page: query.page,
                per_page: Some(per_page as u32),
                approved_state: if user_can_see_unnaproved_articles {
                    query.approved_state
                } else {
                    Some(false)
                },
                query: if let Some(author) = query.author {
                    Some(FetchArticleQuery::Author(author))
                } else {
                    query.title.map(FetchArticleQuery::Title)
                },
            })
            .await
            .map(|articles| {
                ArticlePreviewPresenter::to_json_paginated_wrapper(
                    articles.data,
                    articles.pagination,
                    per_page,
                )
            })
    }
}
