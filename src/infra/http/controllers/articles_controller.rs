use actix_web::{http::StatusCode, web, HttpResponse, HttpResponseBuilder, Responder};
use actix_web_lab::middleware::from_fn;
use serde_json::json;

use crate::{domain::{factories::create_article_service_factory, services::create_article_service::CreateArticleParams}, infra::http::{dtos::create_article::CreateArticleDto, extractors::req_user::ReqUser, middlewares::authentication_middleware, presenters::{article::ArticlePresenter, error::ErrorPresenter}}};

use super::controller::ControllerTrait;

pub struct ArticlesController;

impl ControllerTrait for ArticlesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/articles")
            // CREATE
            .route("/new", web::post().to(Self::create)).wrap(from_fn(authentication_middleware))

            // READ
            .route("/{id}/get", web::get().to(Self::get))
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update))

            // DELETE
            .route("/{id}/delete", web::put().to(Self::delete))
        );
    }
}

impl ArticlesController {
    async fn create(
        body: web::Json<CreateArticleDto>,
        user: web::ReqData<ReqUser>
    ) -> impl Responder {
        let auth_user = user.into_inner();

        let service = create_article_service_factory::exec().await;

        let CreateArticleDto {author_id, content, cover_url, title} = body.into_inner();

        let result = service.exec(CreateArticleParams {
            custom_author_id: author_id,
            staff_id: auth_user.user_id,
            content,
            cover_url,
            title
        }).await;
                
        if result.is_err() {
            let err = result.unwrap_err();

            return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
            .json(ErrorPresenter::to_http(err));
        }

        let article = result.unwrap();
        let mapped_article = ArticlePresenter::to_http(article);

        return HttpResponse::Created().json(json!({"data": mapped_article}));
    }

    async fn get() -> impl Responder {
        return HttpResponse::Ok().finish();
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
