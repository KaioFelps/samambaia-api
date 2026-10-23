use actix_session::Session;
use actix_web::web::{self, Data, Json, Path, Query, Redirect, ThinData};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use jobs::job::Job;
use jobs::logging::{
    ActionKind,
    Actor,
    EntitySnapshot,
    LOGGING_JOB_TYPE,
    LoggingJobBuilder,
    Target,
    TargetType,
};
use jobs::queue::JobQueue;
use serde::Serialize;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::domain_entities::user::User;
use crate::domain::factories::journalism::article_tags::{
    create_article_tag_service_factory,
    delete_article_tag_service_factory,
    fetch_many_article_tags_service_factory,
    update_article_tag_service_factory,
};
use crate::domain::services::journalism::article_tags::create_article_tag_service::CreateArticleTagParams;
use crate::domain::services::journalism::article_tags::delete_article_tag_service::DeleteArticleTagParams;
use crate::domain::services::journalism::article_tags::fetch_many_article_tags_service::FetchManyArticleTagsParams;
use crate::domain::services::journalism::article_tags::update_article_tag_service::UpdateArticleTagParams;
use crate::error::{IntoSamambaiaError, SamambaiaError};
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::{AppResponse, AppResponseRedirect};
use crate::infra::http::dtos::create_article_tag::CreateArticleTagDto;
use crate::infra::http::dtos::list_article_tags::ListArticleTagsDto;
use crate::infra::http::dtos::update_article_tag::UpdateArticleTagDto;
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::middlewares::web::has_permission::{
    PermissionComparisonMode,
    WebHasPermissionMiddleware,
};
use crate::infra::http::presenters::article_tag::ArticleTagPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::http::presenters::user::UserPresenter;
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
                        .to(Self::manage)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticle],
                            PermissionComparisonMode::Any,
                        )),
                )
                .route(
                    "nova",
                    web::get()
                        .to(Self::create)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticleTag],
                            PermissionComparisonMode::Any,
                        )),
                )
                .route(
                    "criar",
                    web::post()
                        .to(Self::store)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::CreateArticleTag],
                            PermissionComparisonMode::Any,
                        )),
                )
                .route(
                    "{tag_id}/atualizar",
                    web::put()
                        .to(Self::update)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticleTag],
                            PermissionComparisonMode::Any,
                        )),
                )
                .route(
                    "{tag_id}/apagar",
                    web::delete()
                        .to(Self::delete)
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::DeleteArticleTag],
                            PermissionComparisonMode::Any,
                        )),
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
        job_queue: ThinData<JobQueue>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Err(redirect) => return Ok(redirect),
            Ok(body) => body,
        };

        let create_article_tag_service = create_article_tag_service_factory::exec(&db_conn);

        let tag = create_article_tag_service
            .exec(CreateArticleTagParams {
                value: body.value.clone(),
                user_role: auth_user.user.role().unwrap(),
            })
            .await?;

        enqueue_article_tag_audit_log(
            &job_queue,
            ActionKind::Create,
            &auth_user.user,
            &tag,
            None::<&()>,
            Some(&body),
        )
        .await;

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

    async fn update(
        req: HttpRequest,
        tag_id: Path<u32>,
        body: Json<UpdateArticleTagDto>,
        db_conn: Data<SeaService>,
        auth_user: WebAuthUser,
        job_queue: ThinData<JobQueue>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Err(redirect) => return Ok(redirect),
            Ok(body) => body,
        };

        let update_tag_service = update_article_tag_service_factory::exec(&db_conn);

        let tag_update = update_tag_service
            .exec(UpdateArticleTagParams {
                tag_id: tag_id.into_inner() as i32,
                user_role: auth_user.user.role().unwrap(),
                value: body.value.clone(),
            })
            .await;

        let response = match tag_update {
            Ok(tag) => {
                Session::flash_silently(
                    &req,
                    "updateArticleTagSuccess",
                    format!("Tag {} atualizada com sucesso!", tag.value()),
                );

                enqueue_article_tag_audit_log(
                    &job_queue,
                    ActionKind::Edit,
                    &auth_user.user,
                    &tag,
                    None::<&()>,
                    Some(&body),
                )
                .await;

                Inertia::back(&req)
            }
            Err(err) => {
                if matches!(&err, SamambaiaError::BadRequest(_)) {
                    Session::flash_silently(
                        &req,
                        "updateArticleTagSuccess",
                        "Não havia nada para atualizar.",
                    );

                    Inertia::back(&req)
                } else {
                    let error_msg = match err {
                        SamambaiaError::Unauthorized(msg) => msg.to_string().into(),
                        _ => "Não foi possível atualizar esta tag. Contate um desenvolvedor."
                            .to_string()
                            .into(),
                    };

                    Inertia::back_with_errors(&req, hashmap!["error" => error_msg])
                }
            }
        };

        Ok(response)
    }

    async fn delete(
        req: HttpRequest,
        tag_id: Path<u32>,
        auth_user: WebAuthUser,
        db_conn: Data<SeaService>,
        job_queue: ThinData<JobQueue>,
    ) -> Redirect {
        let service = delete_article_tag_service_factory::exec(&db_conn);

        match service
            .exec(DeleteArticleTagParams {
                tag_id: tag_id.into_inner() as i32,
                user_role: auth_user.user.role().as_ref().unwrap(),
            })
            .await
        {
            Err(err) => Inertia::back_with_errors(
                &req,
                hashmap!["error" => match err {
                    SamambaiaError::Unauthorized(msg) => msg.to_string().into(),
                    _ => "Não foi possível deletar esta tag. Contate um desenvolvedor.".to_string().into()
                }],
            ),

            Ok(tag) => {
                if let Some(tag) = tag {
                    enqueue_article_tag_audit_log(
                        &job_queue,
                        ActionKind::Delete,
                        &auth_user.user,
                        &tag,
                        None::<&()>,
                        None::<&()>,
                    )
                    .await;
                }

                Session::flash_silently(
                    &req,
                    "deleteArticleTagSuccess",
                    "Tag deletada com sucesso.",
                );

                Inertia::back(&req)
            }
        }
    }
}

async fn enqueue_article_tag_audit_log<OldDto, NewDto>(
    job_queue: &JobQueue,
    action: ActionKind,
    user: &User, // Passando por referência para não consumir o owner
    tag: &ArticleTag,
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
        tag.id().to_string(),
        tag.value().to_string(),
        ArticleTagPresenter::to_http(tag.clone()),
    ) else {
        log::error!("Failed to create ArticleTag snapshot for audit log");
        return;
    };

    let mut builder = LoggingJobBuilder::default()
        .action_kind(action.clone())
        .actor(Actor::Entity(actor_snapshot))
        .target(Target {
            t_type: TargetType::ArticleTag,
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
            let log_job = Job::new(LOGGING_JOB_TYPE, log_data);
            if let Err(err) = job_queue.enqueue(log_job).await {
                log::error!(
                    "Error enqueueing audit log ({:?}) for ArticleTag: {err:?}",
                    action
                );
            }
        }
        Err(err) => {
            log::error!(
                "Error building audit log ({:?}) for ArticleTag: {err}",
                action
            );
        }
    }
}
