use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use serde_json::json;
use either::Either::*;
use validator::Validate;

use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::slug::Slug;
use crate::domain::factories::{create_article_service_factory, fetch_many_articles_service_factory, get_expanded_article_service_factory};
use crate::domain::services::create_article_service::CreateArticleParams;
use crate::domain::services::fetch_many_articles_service::{FetchManyArticlesParams, ServiceArticleQueryType};
use crate::domain::services::get_expanded_article_service::{FetchManyCommentsWithAuthorResponse, GetExpandedArticleParams, GetExpandedArticleResponse};
use crate::infra::http::dtos::create_article::CreateArticleDto;
use crate::infra::http::dtos::list_article_admin::AdminListArticlesDto;
use crate::infra::http::dtos::list_articles::ListArticlesDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::article::MappedArticle;
use crate::infra::http::presenters::error::ErrorPresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
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
            .route("/{slug}/get", web::get().to(Self::get))
            .route("/list", web::get().to(Self::list))
            .route("/list/admin", web::get().to(Self::admin_list).wrap(from_fn(authentication_middleware)))
            
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

        let service = match create_article_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

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

    async fn get(
        article_slug: web::Path<String>,
        user: Option<web::ReqData<ReqUser>>
    ) -> impl Responder {
        let service = match get_expanded_article_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

        let user = match user {
            None => None,
            Some(user) => Some(user)
        };

        let (user_id, user_role) = match &user {
            None => (None, None),
            Some(user) => (Some(&user.user_id), Some(user.user_role.as_ref().unwrap()))
        };

        let result = service.exec(GetExpandedArticleParams {
            article_slug: Slug::new_from_existing(article_slug.into_inner()),
            comments_per_page: Some(DEFAULT_PER_PAGE as u32),
            user_id,
            user_role,
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

    async fn list(body: web::Json<ListArticlesDto>) -> impl Responder {
        let req_body = match body.validate() {
            Ok(()) => body.into_inner(),
            Err(err) => {
                return HttpResponse::BadRequest()
                .json(ErrorPresenter::to_http_from_validator(err.field_errors()));
            }
        };

        return Self::get_list_of_articles(
            req_body.title,
            req_body.author,
            req_body.page,
            req_body.per_page,
            Some(true)
        ).await;
    }

    async fn admin_list(body: web::Json<AdminListArticlesDto>) -> impl Responder {
        let req_body = match body.validate() {
            Ok(()) => body.into_inner(),
            Err(err) => {
                return HttpResponse::BadRequest()
                .json(ErrorPresenter::to_http_from_validator(err.field_errors()));
            }
        };

        return Self::get_list_of_articles(
            req_body.title,
            req_body.author,
            req_body.page,
            req_body.per_page,
            req_body.approved_state,
        ).await;
    }

    async fn update() -> impl Responder {
        return HttpResponse::NoContent().finish();
    }

    async fn delete() -> impl Responder {
        return HttpResponse::NoContent().finish();
    }

    async fn get_list_of_articles(title: Option<String>, author: Option<String>, page: Option<u32>, per_page: Option<u8>, approved_state: Option<bool>) -> HttpResponse {
        let service = match fetch_many_articles_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

        let query = {
            if title.is_some() {
                Some(ServiceArticleQueryType::Title(title.unwrap()))
            } else if author.is_some() {
                Some(ServiceArticleQueryType::Author(author.unwrap()))
            } else {   
                None
            }
        };

        let result = service.exec(FetchManyArticlesParams {
            page,
            per_page: if per_page.is_some() { Some(per_page.unwrap() as u32) } else { None },
            query,
            approved_state,
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let result = result.unwrap();

        let mapped_articles = result.data.into_iter().map(ArticlePresenter::to_http).collect::<Vec<MappedArticle>>();

        return HttpResponse::Ok().json(json!({
            "pagination": PaginationPresenter::to_http(result.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE)),
            "data": mapped_articles
        }));
    }
}
