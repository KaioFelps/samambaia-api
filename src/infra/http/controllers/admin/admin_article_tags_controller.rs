use actix_session::Session;
use actix_web::web::{self, Data, Json, Path, Query, Redirect};
use actix_web::{Either, HttpRequest};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::journalism::article_tags::{
    create_article_tag_service_factory,
    delete_article_tag_service_factory,
    fetch_many_article_tags_service_factory,
    find_article_tag_by_id_service_factory,
    update_article_tag_service_factory,
};
use crate::domain::services::journalism::article_tags::create_article_tag_service::CreateArticleTagParams;
use crate::domain::services::journalism::article_tags::delete_article_tag_service::DeleteArticleTagParams;
use crate::domain::services::journalism::article_tags::fetch_many_article_tags_service::FetchManyArticleTagsParams;
use crate::domain::services::journalism::article_tags::find_article_tag_by_id_service::FindArticleTagByIdParams;
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
                )
                .route(
                    "{tag_id}/editar",
                    web::get()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticleTag],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::edit),
                )
                .route(
                    "{tag_id}/atualizar",
                    web::put()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::UpdateArticleTag],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::update),
                )
                .route(
                    "{tag_id}/apagar",
                    web::delete()
                        .wrap(WebHasPermissionMiddleware::new(
                            vec![RolePermissions::DeleteArticleTag],
                            PermissionComparisonMode::Any,
                        ))
                        .to(Self::delete),
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

    async fn edit(req: HttpRequest, tag_id: Path<u32>, db_conn: Data<SeaService>) -> AppResponse {
        let find_tag_service = find_article_tag_by_id_service_factory::exec(&db_conn);

        let tag_id = tag_id.into_inner();
        let tag = find_tag_service
            .exec(FindArticleTagByIdParams { tag_id })
            .await
            .map(|response| response.tag.map(ArticleTagPresenter::to_http))?;

        Inertia::render_with_props(
            &req,
            "admin/articles/tags/edit".into(),
            hashmap![
                "tag" => InertiaProp::data(tag)
            ],
        )
        .await
        .map_err(IntoSamambaiaError::into_samambaia_error)
    }

    async fn update(
        req: HttpRequest,
        tag_id: Path<u32>,
        body: Json<UpdateArticleTagDto>,
        db_conn: Data<SeaService>,
        auth_user: WebAuthUser,
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
                value: body.value,
            })
            .await;

        let response = match tag_update {
            Ok(tag) => {
                Session::flash_silently(
                    &req,
                    "updateArticleTagSuccess",
                    format!("Tag {} atualizada com sucesso!", tag.value()),
                );

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
    ) -> Redirect {
        let service = delete_article_tag_service_factory::exec(&db_conn);

        if let Err(err) = service
            .exec(DeleteArticleTagParams {
                tag_id: tag_id.into_inner() as i32,
                user_role: auth_user.user.role().as_ref().unwrap(),
            })
            .await
        {
            return Inertia::back_with_errors(
                &req,
                hashmap!["error" => match err {
                    SamambaiaError::Unauthorized(msg) => msg.to_string().into(),
                    _ => "Não foi possível deletar esta tag. Contate um desenvolvedor.".to_string().into()
                }],
            );
        }

        Session::flash_silently(&req, "deleteArticleTagSuccess", "Tag deletada com sucesso.");

        Inertia::back(&req)
    }
}
