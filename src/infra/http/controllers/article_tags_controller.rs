use super::controller::ControllerTrait;
use super::AppResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::{
    create_article_tag_service_factory, delete_article_tag_service_factory,
    fetch_many_article_tags_service_factory, update_article_tag_service_factory,
};
use crate::domain::services::journalism::article_tags::{
    create_article_tag_service::CreateArticleTagParams,
    delete_article_tag_service::DeleteArticleTagParams,
    fetch_many_article_tags_service::FetchManyArticleTagsParams,
    update_article_tag_service::UpdateArticleTagParams,
};
use crate::infra::extensions::validator::IntoDomainError;
use crate::infra::http::dtos::create_article_tag::CreateArticleTagDto;
use crate::infra::http::dtos::list_article_tags::ListArticleTagsDto;
use crate::infra::http::dtos::update_article_tag::UpdateArticleTagDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::article_tag::{ArticleTagPresenter, MappedArticleTag};
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::infra::sea::sea_service::SeaService;
use actix_web::{middleware::from_fn, web, HttpResponse};
use validator::Validate;

pub struct ArticleTagsController;

impl ControllerTrait for ArticleTagsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/article_tags")
                // CREATE
                .route(
                    "/new",
                    web::post()
                        .to(Self::create)
                        .wrap(from_fn(authentication_middleware)),
                )
                // READ
                .route("/list", web::get().to(Self::list))
                // UPDATE
                .route(
                    "/{id}/update",
                    web::put()
                        .to(Self::update)
                        .wrap(from_fn(authentication_middleware)),
                )
                // DELETE
                .route(
                    "/{id}/delete",
                    web::delete()
                        .to(Self::delete)
                        .wrap(from_fn(authentication_middleware)),
                ),
        );
    }
}

impl ArticleTagsController {
    async fn create(
        db_conn: web::Data<SeaService>,
        body: web::Json<CreateArticleTagDto>,
        user: web::ReqData<ReqUser>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoDomainError::into_domain_err)?;

        let service = create_article_tag_service_factory::exec(&db_conn);

        let article_tag = service
            .exec(CreateArticleTagParams {
                user_role: user.into_inner().user_role.unwrap(),
                value: body.value,
            })
            .await?;

        let mapped_article_tag = ArticleTagPresenter::to_http(article_tag);

        Ok(HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_article_tag,
        }))
    }

    async fn list(
        db_conn: web::Data<SeaService>,
        query: web::Query<ListArticleTagsDto>,
    ) -> AppResponse {
        let ListArticleTagsDto {
            page,
            per_page,
            value,
        } = query
            .validate()
            .map(|_| query.into_inner())
            .map_err(IntoDomainError::into_domain_err)?;

        let service = fetch_many_article_tags_service_factory::exec(&db_conn);

        let service_response = service
            .exec(FetchManyArticleTagsParams {
                per_page: per_page.map(|pp| pp as u32),
                query: value,
                page,
            })
            .await?;

        let mapped_article_tags = service_response
            .data
            .into_iter()
            .map(ArticleTagPresenter::to_http)
            .collect::<Vec<MappedArticleTag>>();

        let mapped_pagination = PaginationPresenter::to_http(
            service_response.pagination,
            per_page.unwrap_or(DEFAULT_PER_PAGE),
        );

        Ok(
            HttpResponse::Ok().json(ArticleTagPresenter::to_json_paginated_wrapper(
                mapped_article_tags,
                mapped_pagination,
            )),
        )
    }

    async fn update(
        db_conn: web::Data<SeaService>,
        body: web::Json<UpdateArticleTagDto>,
        user: web::ReqData<ReqUser>,
        tag_id: web::Path<i32>,
    ) -> AppResponse {
        let body = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(IntoDomainError::into_domain_err)?;

        let service = update_article_tag_service_factory::exec(&db_conn);

        let article_tag = service
            .exec(UpdateArticleTagParams {
                value: body.value,
                tag_id: tag_id.into_inner(),
                user_role: user.into_inner().user_role.unwrap(),
            })
            .await?;

        let mapped_article_tag = ArticleTagPresenter::to_http(article_tag);

        Ok(HttpResponse::NoContent().json(JsonWrappedEntity {
            data: mapped_article_tag,
        }))
    }

    async fn delete(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        tag_id: web::Path<i32>,
    ) -> AppResponse {
        let service = delete_article_tag_service_factory::exec(&db_conn);

        service
            .exec(DeleteArticleTagParams {
                user_role: user.user_role.as_ref().unwrap(),
                tag_id: tag_id.into_inner(),
            })
            .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
