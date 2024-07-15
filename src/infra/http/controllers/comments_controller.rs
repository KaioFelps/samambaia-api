use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use either::{Right, Left};
use serde_json::json;
use uuid::Uuid;
use crate::core::pagination::DEFAULT_PER_PAGE;
use super::controller::ControllerTrait;
use crate::domain::factories::{comment_on_article_service_factory, delete_comment_service_factory, fetch_many_comments_with_author_service_factory};
use crate::domain::factories::fetch_many_comments_service_factory;
use crate::domain::factories::toggle_comment_visibility_service_factory;
use crate::domain::services::comment_on_article_service::CommentOnArticleParams;
use crate::domain::services::delete_comment_service::DeleteCommentParams;
use crate::domain::services::fetch_many_comments_service::{FetchManyCommentsParams, ServiceCommentQueryType};
use crate::domain::services::fetch_many_comments_with_author_service::FetchManyArticleCommentsWithAuthorParams;
use crate::domain::services::toggle_comment_visibility_service::ToggleCommentVisibilityParams;
use crate::infra::http::dtos::comment_on_article::CommentOnArticleDto;
use crate::infra::http::dtos::list_comments::ListCommentsDto;
use crate::infra::http::dtos::simple_pagination_query::SimplePaginationQueryDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::comment::{CommentPresenter, MappedComment, MappedRawComment};
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::util::generate_error_response;

pub struct CommentsController;

impl ControllerTrait for CommentsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/comments")
            // Comment on an article
            .route("/{article_id}/new", web::post().to(Self::create).wrap(from_fn(authentication_middleware)))
            // Get an article's comments with author list
            .route("/{article_id}/list", web::get().to(Self::list))
            // Get a comments list
            .route("/list/admin", web::get().to(Self::admin_list).wrap(from_fn(authentication_middleware)))
            // Deactivate comment visibility
            .route("/{id}/deactivate", web::patch().to(Self::disable_visibility).wrap(from_fn(authentication_middleware)))
            // Definitely delete a comment
            .route("/{id}/delete", web::delete().to(Self::delete).wrap(from_fn(authentication_middleware)))
        );
    }
}

impl CommentsController {
    async fn create(
        article_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>,
        body: web::Json<CommentOnArticleDto>
    ) -> impl Responder {
        let service = match comment_on_article_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

        let result = service.exec(CommentOnArticleParams {
            author_id: user.user_id,
            content: body.into_inner().content,
            article_id: article_id.into_inner()
        }).await;

        if result.is_err() {
            let error = result.unwrap_err();
            return generate_error_response(error);
        }

        return HttpResponse::Created().finish();
    }

    async fn list(
        article_id: web::Path<Uuid>,
        query: web::Query<SimplePaginationQueryDto>
    ) -> impl Responder {
        let service = match fetch_many_comments_with_author_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

        let SimplePaginationQueryDto {
            per_page,
            page
        } = query.into_inner();

        let result = service.exec(article_id.into_inner(), FetchManyArticleCommentsWithAuthorParams {
            page,
            per_page: if per_page.is_some() { Some(per_page.unwrap() as u32) } else { None },
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let result = result.unwrap();
        let mapped_comments: Vec<MappedComment> = result.data.into_iter().map(CommentPresenter::to_http).collect();

        return HttpResponse::Ok().json(json!({
            "pagination": PaginationPresenter::to_http(result.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE)),
            "data": mapped_comments
        }));
    }

    async fn admin_list(
        query: web::Query<ListCommentsDto>
    ) -> impl Responder {
        let service = match fetch_many_comments_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error
        };

        let ListCommentsDto {
            page,
            per_page,
            author,
            content,
            include_inactive
        } = query.into_inner();

        let include_inactive = if include_inactive.is_some() {
            include_inactive.unwrap()
        } else {
            false
        };

        let query = {
            if author.is_some() { Some(ServiceCommentQueryType::Author(author.unwrap())) }
            else if content.is_some() { Some(ServiceCommentQueryType::Content(content.unwrap())) }
            else { None }
        };

        let result = service.exec(include_inactive, FetchManyCommentsParams {
            query,
            per_page: if per_page.is_none() { None } else { Some(per_page.unwrap() as u32) },
            page,
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        let result = result.unwrap();

        let mapped_comments: Vec<MappedRawComment> = result.data.into_iter().map(CommentPresenter::to_http_raw).collect();

        return HttpResponse::Ok().json(json!({
            "pagination": PaginationPresenter::to_http(result.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE)),
            "data": mapped_comments
        }));
    }

    async fn disable_visibility(
        user: web::ReqData<ReqUser>,
        comment_id: web::Path<Uuid>
    ) -> impl Responder {
        let service = match toggle_comment_visibility_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let user_role = &user.user_role;

        let result = service.exec(ToggleCommentVisibilityParams {
            user_role: user_role.as_ref().unwrap(),
            comment_id: comment_id.into_inner(),
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }

    async fn delete(
        comment_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>
    ) -> impl Responder {
        let service = match delete_comment_service_factory::exec().await {
            Left(service) => service,
            Right(error) => return error,
        };

        let ReqUser {user_role, user_id, exp: _} = user.into_inner();

        let result = service.exec(DeleteCommentParams {
            comment_id: comment_id.into_inner(),
            user_id,
            staff_role: user_role.unwrap(),
        }).await;

        if result.is_err() {
            return generate_error_response(result.unwrap_err());
        }

        return HttpResponse::NoContent().finish();
    }
}
