use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Left, Right};
use validator::Validate;
use super::controller::ControllerTrait;
use crate::domain::factories::{
    delete_article_tag_service_factory,
    fetch_many_article_tags_service_factory,
    update_article_tag_service_factory,
    create_article_tag_service_factory
};
use crate::domain::services::create_article_tag_service::CreateArticleTagParams;
use crate::infra::http::dtos::create_article_tag::CreateArticleTagDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::article_tag::ArticleTagPresenter;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::presenter::{JsonWrappedEntity, PresenterTrait};
use crate::util::generate_error_response;

pub struct ArticleTagsController;

impl ControllerTrait for ArticleTagsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/article_tags")
            // CREATE
            .route("/new", web::post().to(Self::create).wrap(from_fn(authentication_middleware)))

            // READ
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update).wrap(from_fn(authentication_middleware)))

            // DELETE
            .route("/{id}/delete", web::delete().to(Self::delete).wrap(from_fn(authentication_middleware)))
        );
    }
}

impl ArticleTagsController {
    async fn create(body: web::Json<CreateArticleTagDto>, user: web::ReqData<ReqUser>) -> impl Responder {
        match body.validate() {
            Ok(()) => (),
            Err(error) => return HttpResponse::BadRequest().json(ErrorPresenter::to_http_from_validator(error.field_errors())),
        };

        let body = body.into_inner();

        let service = match create_article_tag_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let result = service.exec(CreateArticleTagParams {
            user_role: user.into_inner().user_role.unwrap(),
            value: body.value
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let article_tag = result.unwrap();
        let mapped_article_tag = ArticleTagPresenter::to_http(article_tag);

        return HttpResponse::Created().json(JsonWrappedEntity {
            data: mapped_article_tag
        });
    }

    async fn list() -> impl Responder {
        return HttpResponse::Ok().finish();
    }

    async fn update() -> impl Responder {
        return HttpResponse::NoContent().finish();
    }

    async fn delete() -> impl Responder {
        return HttpResponse::NoContent().finish();
    }
}
