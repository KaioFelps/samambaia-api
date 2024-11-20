use super::controller::ControllerTrait;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::{
    create_article_tag_service_factory, delete_article_tag_service_factory,
    fetch_many_article_tags_service_factory, update_article_tag_service_factory,
};
use crate::domain::services::create_article_tag_service::CreateArticleTagParams;
use crate::domain::services::delete_article_tag_service::DeleteArticleTagParams;
use crate::domain::services::fetch_many_article_tags_service::FetchManyArticleTagsParams;
use crate::domain::services::update_article_tag_service::UpdateArticleTagParams;
use crate::infra::http::dtos::create_article_tag::CreateArticleTagDto;
use crate::infra::http::dtos::list_article_tags::ListArticleTagsDto;
use crate::infra::http::dtos::update_article_tag::UpdateArticleTagDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::article_tag::{ArticleTagPresenter, MappedArticleTag};
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::util::generate_error_response;
use actix_web::{middleware::from_fn, web, HttpResponse, Responder};
use either::{Left, Right};
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
        body: web::Json<CreateArticleTagDto>,
        user: web::ReqData<ReqUser>,
    ) -> impl Responder {
        match body.validate() {
            Ok(()) => (),
            Err(error) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(error.field_errors()))
            }
        };

        let body = body.into_inner();

        let service = match create_article_tag_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let article_tag = match service
            .exec(CreateArticleTagParams {
                user_role: user.into_inner().user_role.unwrap(),
                value: body.value,
            })
            .await
        {
            Err(err) => return generate_error_response(err),
            Ok(article_tag) => article_tag,
        };

        let mapped_article_tag = ArticleTagPresenter::to_http(article_tag);

        HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_article_tag,
        })
    }

    async fn list(query: web::Query<ListArticleTagsDto>) -> impl Responder {
        match query.validate() {
            Ok(()) => (),
            Err(error) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(error.field_errors()))
            }
        };

        let ListArticleTagsDto {
            page,
            per_page,
            value,
        } = query.into_inner();

        let service = match fetch_many_article_tags_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let service_response = match service
            .exec(FetchManyArticleTagsParams {
                per_page: per_page.map(|pp| pp as u32),
                query: value,
                page,
            })
            .await
        {
            Err(err) => return generate_error_response(err),
            Ok(service_res) => service_res,
        };

        let mapped_article_tags = service_response
            .data
            .into_iter()
            .map(ArticleTagPresenter::to_http)
            .collect::<Vec<MappedArticleTag>>();

        let mapped_pagination = PaginationPresenter::to_http(
            service_response.pagination,
            per_page.unwrap_or(DEFAULT_PER_PAGE),
        );

        HttpResponse::Ok().json(ArticleTagPresenter::to_json_paginated_wrapper(
            mapped_article_tags,
            mapped_pagination,
        ))
    }

    async fn update(
        body: web::Json<UpdateArticleTagDto>,
        user: web::ReqData<ReqUser>,
        tag_id: web::Path<i32>,
    ) -> impl Responder {
        match body.validate() {
            Ok(()) => (),
            Err(error) => {
                return HttpResponse::BadRequest()
                    .json(ErrorPresenter::to_http_from_validator(error.field_errors()))
            }
        };

        let body = body.into_inner();

        let service = match update_article_tag_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let article_tag = match service
            .exec(UpdateArticleTagParams {
                value: body.value,
                tag_id: tag_id.into_inner(),
                user_role: user.into_inner().user_role.unwrap(),
            })
            .await
        {
            Err(err) => return generate_error_response(err),
            Ok(article_tag) => article_tag,
        };

        let mapped_article_tag = ArticleTagPresenter::to_http(article_tag);

        HttpResponse::NoContent().json(JsonWrappedEntity {
            data: mapped_article_tag,
        })
    }

    async fn delete(user: web::ReqData<ReqUser>, tag_id: web::Path<i32>) -> impl Responder {
        let service = match delete_article_tag_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let service_response = service
            .exec(DeleteArticleTagParams {
                user_role: user.user_role.as_ref().unwrap(),
                tag_id: tag_id.into_inner(),
            })
            .await;

        if let Err(err) = service_response {
            return generate_error_response(err);
        }

        HttpResponse::NoContent().finish()
    }
}
