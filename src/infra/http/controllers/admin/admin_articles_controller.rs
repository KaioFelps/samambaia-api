use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{self, Data, Json, Path, Query, Redirect, ThinData};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use jobs::logging::{ActionKind, Actor, EntitySnapshot, LoggingJobBuilder, Target, TargetType};
use jobs::queue::JobQueue;
use serde::Serialize;
use uuid::Uuid;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::article::Article;
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
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::presenters::article::ArticlePresenter;
use crate::infra::http::presenters::article_preview::{
    ArticlePreviewPresenter,
    MappedArticlePreview,
};
use crate::infra::http::presenters::article_tag::ArticleTagPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedPaginatedEntity, PresenterTrait};
use crate::infra::http::presenters::user::UserPresenter;
use crate::infra::sea::sea_service::SeaService;
use crate::util::{RolePermissions, verify_role_has_permission};

pub struct AdminArticlesController;

impl ControllerTrait for AdminArticlesController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/noticias")
                .route(
                    "",
                    web::get()
                        .to(Self::manage_articles)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![
                                RolePermissions::SeeUnapprovedArticle,
                                RolePermissions::CreateArticle,
                            ],
                            PermissionComparisonMode::Any,
                        )),
                )
                .route(
                    "nova",
                    web::get()
                        .to(Self::create_article)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticle],
                            PermissionComparisonMode::All,
                        )),
                )
                .route(
                    "criar",
                    web::post()
                        .to(Self::store_article)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticle],
                            PermissionComparisonMode::All,
                        )),
                )
                .route(
                    "{article_id}/alterar-aprovado",
                    web::patch().to(Self::toggle_article_approved).wrap(
                        WebHasPermissionMiddleware::new(
                            vec![RolePermissions::ApproveArticle],
                            PermissionComparisonMode::Any,
                        ),
                    ),
                )
                .route(
                    "{article_id}/editar",
                    web::get()
                        .to(Self::edit_article)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticle],
                            PermissionComparisonMode::All,
                        )),
                )
                .route(
                    "{article_id}/atualizar",
                    web::put()
                        .to(Self::update_article)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticle],
                            PermissionComparisonMode::All,
                        )),
                )
                .route(
                    "{article_id}/apagar",
                    web::delete()
                        .to(Self::delete_article)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![
                                RolePermissions::DeleteArticle,
                                RolePermissions::DeleteComment,
                            ],
                            PermissionComparisonMode::All,
                        )),
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
        job_queue: ThinData<JobQueue>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let service = create_article_service_factory::exec(&db_conn);
        let m_body = body.clone();
        let article = service
            .exec(CreateArticleParams {
                title: m_body.title.unwrap(),
                content: m_body.content.unwrap(),
                cover_url: m_body.cover_url.unwrap(),
                custom_author_id: m_body.author_id,
                description: m_body.description.unwrap(),
                staff: &auth.user,
                tags: m_body.tags,
                script: m_body.script,
                cleanup_script: m_body.cleanup_script,
            })
            .await?;

        enqueue_article_audit_log(
            &job_queue,
            ActionKind::Create,
            &auth.user,
            &article,
            None::<&()>,
            Some(&body),
        )
        .await;

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
        job_queue: ThinData<JobQueue>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let article_id = article_id.into_inner();
        let service = update_article_service_factory::exec(&db_conn);

        match service
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
                script: None,
                cleanup_script: None,
            })
            .await
        {
            Err(err) => Ok(Inertia::back_with_errors(
                &req,
                hashmap![
                   "error" => err.get_message().to_string().into()
                ],
            )),
            Ok(article) => {
                enqueue_article_audit_log(
                    &job_queue,
                    ActionKind::Edit,
                    &auth.user,
                    &article,
                    None::<&()>,
                    Some(&body),
                )
                .await;

                Ok(Inertia::back(&req))
            }
        }
    }

    async fn update_article(
        req: HttpRequest,
        auth: WebAuthUser,
        path: Path<Uuid>,
        body: Json<UpdateArticleDto>,
        db_conn: Data<SeaService>,
        job_queue: ThinData<JobQueue>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let service = update_article_service_factory::exec(&db_conn);

        let m_body = body.clone();
        match service
            .exec(UpdateArticleParams {
                title: m_body.title,
                description: m_body.description,
                content: m_body.content,
                approved: m_body.approved,
                cover_url: m_body.cover_url,
                article_id: path.into_inner(),
                author_id: m_body.author_id,
                user: &auth.user,
                tags: m_body.tags,
                script: m_body.script,
                cleanup_script: m_body.cleanup_script,
            })
            .await
        {
            Err(err) => Ok(Inertia::back_with_errors(
                &req,
                hashmap![
                    "error" => err.get_message().to_string().into()
                ],
            )),
            Ok(article) => {
                enqueue_article_audit_log(
                    &job_queue,
                    ActionKind::Edit,
                    &auth.user,
                    &article,
                    None::<&()>,
                    Some(&body),
                )
                .await;

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
        job_queue: ThinData<JobQueue>,
    ) -> AppResponse<Redirect> {
        let service = delete_article_service_factory::exec(&db_conn);

        match service
            .exec(DeleteArticleParams {
                article_id: path.into_inner(),
                user: &auth.user,
            })
            .await
        {
            Err(err) => Ok(Inertia::back_with_errors(
                &req,
                hashmap![
                    "error" => err.get_message().to_string().into()
                ],
            )),

            Ok(article) => {
                if let Some(article) = article {
                    enqueue_article_audit_log(
                        &job_queue,
                        ActionKind::Delete,
                        &auth.user,
                        &article,
                        None::<&()>,
                        None::<&()>,
                    )
                    .await;
                }

                Session::flash_silently(
                    &req,
                    "deleteArticleSuccess",
                    "Notícia apagada com sucesso!",
                );
                Ok(Inertia::back(&req))
            }
        }
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

async fn enqueue_article_audit_log<OldDto, NewDto>(
    job_queue: &JobQueue,
    action: ActionKind,
    user: &User, // Passando por referência para não consumir o owner
    article: &Article,
    old_payload: Option<&OldDto>,
    new_payload: Option<&NewDto>,
) where
    OldDto: Serialize,
    NewDto: Serialize,
{
    let Ok(actor_snapshot) = EntitySnapshot::new(
        user.id().to_string(),
        user.nickname().to_string(),
        UserPresenter::to_http(user.clone()),
    ) else {
        log::error!("Failed to create User snapshot for audit log");
        return;
    };

    let Ok(target_snapshot) = EntitySnapshot::new(
        article.id().to_string(),
        article.title().to_string(),
        ArticlePresenter::to_http(article.clone()),
    ) else {
        log::error!("Failed to create Article snapshot for audit log");
        return;
    };

    let mut builder = LoggingJobBuilder::default()
        .action_kind(action.clone())
        .actor(Actor::Entity(actor_snapshot))
        .target(Target {
            t_type: TargetType::Article,
            entity: target_snapshot,
        });

    if let Some(old) = old_payload
        && let Ok(val) = serde_json::to_value(old)
    {
        builder = builder.old_payload(val);
    }

    if let Some(new) = new_payload
        && let Ok(val) = serde_json::to_value(new)
    {
        builder = builder.new_payload(val);
    }

    match builder.build() {
        Ok(log_data) => {
            if let Err(err) = job_queue.enqueue(log_data.into_job()).await {
                log::error!(
                    "Error enqueueing audit log ({:?}) for Article: {err:?}",
                    action
                );
            }
        }
        Err(err) => {
            log::error!("Error building audit log ({:?}) for Article: {err}", action);
        }
    }
}
