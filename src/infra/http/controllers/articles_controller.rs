use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use serde_json::json;
use uuid::Uuid;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::{create_article_service_factory, get_expanded_article_service_factory};
use crate::domain::services::create_article_service::CreateArticleParams;
use crate::domain::services::get_expanded_article_service::{FetchManyCommentsWithAuthorResponse, GetExpandedArticleParams, GetExpandedArticleResponse};
use crate::infra::http::dtos::create_article::CreateArticleDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::{article::ArticlePresenter, expanded_article::ExpandedArticlePresenter};
use crate::util::generate_error_response;

use super::controller::ControllerTrait;

pub struct ArticlesController;

impl ControllerTrait for ArticlesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/articles")
            // CREATE
            .route(
                "/new", 
                web::post()
                    .to(Self::create)
                    .wrap(from_fn(authentication_middleware))
            ) 

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
            return generate_error_response(err);
        }

        let article = result.unwrap();
        let mapped_article = ArticlePresenter::to_http(article);

        return HttpResponse::Created().json(json!({"data": mapped_article}));
    }

    async fn get(article_id: web::Path<Uuid>) -> impl Responder {
        let service = get_expanded_article_service_factory::exec().await;

        let result = service.exec(GetExpandedArticleParams {
            article_id: article_id.into_inner(),
            comments_per_page: Some(DEFAULT_PER_PAGE as u32,)
        }).await;

        if result.is_err() {
            let err = result.unwrap_err();
            return generate_error_response(err)
        }

        let GetExpandedArticleResponse { article, article_author, comments: comment_response } = result.unwrap();
        let FetchManyCommentsWithAuthorResponse {data: comments, pagination: comments_pagination} = comment_response;

        let mapped_article = ExpandedArticlePresenter::to_http(
            article,
            article_author,
            comments,
            (comments_pagination, DEFAULT_PER_PAGE)
        );

        return HttpResponse::Ok().json(json!({
            "data": mapped_article,
        }));
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
