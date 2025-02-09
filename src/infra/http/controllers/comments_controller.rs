use actix_web::middleware::from_fn;
use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use super::controller::ControllerTrait;
use super::AppResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::journalism::comments::{
    comment_on_article_service_factory,
    delete_comment_service_factory,
    fetch_many_comments_service_factory,
    fetch_many_comments_with_author_service_factory,
};
use crate::domain::factories::security::toggle_comment_visibility_service_factory;
use crate::domain::services::journalism::comments::comment_on_article_service::CommentOnArticleParams;
use crate::domain::services::journalism::comments::delete_comment_service::DeleteCommentParams;
use crate::domain::services::journalism::comments::fetch_many_comments_service::{
    FetchManyCommentsParams,
    ServiceCommentQueryType,
};
use crate::domain::services::journalism::comments::fetch_many_comments_with_author_service::FetchManyArticleCommentsWithAuthorParams;
use crate::domain::services::security::toggle_comment_visibility_service::ToggleCommentVisibilityParams;
use crate::infra::http::dtos::comment_on_article::CommentOnArticleDto;
use crate::infra::http::dtos::list_comments::ListCommentsDto;
use crate::infra::http::dtos::simple_pagination_query::SimplePaginationQueryDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::comment::{CommentPresenter, MappedComment, MappedRawComment};
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;

pub struct CommentsController;

impl ControllerTrait for CommentsController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/comments")
                // Comment on an article
                .route(
                    "/{article_id}/new",
                    web::post()
                        .to(Self::create)
                        .wrap(from_fn(authentication_middleware)),
                )
                // Get an article's comments with author list
                .route("/{article_id}/list", web::get().to(Self::list))
                // Get a comments list
                .route(
                    "/list/admin",
                    web::get()
                        .to(Self::admin_list)
                        .wrap(from_fn(authentication_middleware)),
                )
                // Deactivate comment visibility
                .route(
                    "/{id}/deactivate",
                    web::patch()
                        .to(Self::disable_visibility)
                        .wrap(from_fn(authentication_middleware)),
                )
                // Definitely delete a comment
                .route(
                    "/{id}/delete",
                    web::delete()
                        .to(Self::delete)
                        .wrap(from_fn(authentication_middleware)),
                ),
        );
    }
}

impl CommentsController {
    async fn create(
        db_conn: web::Data<SeaService>,
        article_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>,
        body: web::Json<CommentOnArticleDto>,
    ) -> AppResponse {
        let service = comment_on_article_service_factory::exec(&db_conn);

        let _comment = service
            .exec(CommentOnArticleParams {
                author_id: user.user_id,
                content: body.into_inner().content,
                article_id: article_id.into_inner(),
            })
            .await?;

        Ok(HttpResponse::Created().finish())
    }

    async fn list(
        db_conn: web::Data<SeaService>,
        article_id: web::Path<Uuid>,
        query: web::Query<SimplePaginationQueryDto>,
    ) -> AppResponse {
        let service = fetch_many_comments_with_author_service_factory::exec(&db_conn);

        let SimplePaginationQueryDto { per_page, page } = query.into_inner();

        let comments = service
            .exec(
                article_id.into_inner(),
                FetchManyArticleCommentsWithAuthorParams {
                    page,
                    per_page: per_page.map(|pp| pp as u32),
                },
            )
            .await?;

        let mapped_comments: Vec<MappedComment> = comments
            .data
            .into_iter()
            .map(CommentPresenter::to_http)
            .collect();

        Ok(HttpResponse::Ok().json(json!({
                    "pagination": PaginationPresenter::to_http(comments.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE)),
                    "data": mapped_comments
                })))
    }

    async fn admin_list(
        db_conn: web::Data<SeaService>,
        query: web::Query<ListCommentsDto>,
    ) -> AppResponse {
        let service = fetch_many_comments_service_factory::exec(&db_conn);

        let ListCommentsDto {
            page,
            per_page,
            author,
            content,
            include_inactive,
        } = query.into_inner();

        let include_inactive = include_inactive.unwrap_or(false);

        let query = {
            if let Some(author) = author {
                Some(ServiceCommentQueryType::Author(author))
            } else {
                content.map(ServiceCommentQueryType::Content)
            }
        };

        let comments = service
            .exec(
                include_inactive,
                FetchManyCommentsParams {
                    query,
                    per_page: per_page.map(|pp| pp as u32),
                    page,
                },
            )
            .await?;

        let mapped_comments: Vec<MappedRawComment> = comments
            .data
            .into_iter()
            .map(CommentPresenter::to_http_raw)
            .collect();

        Ok(HttpResponse::Ok().json(json!({
                    "pagination": PaginationPresenter::to_http(comments.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE)),
                    "data": mapped_comments
                })))
    }

    async fn disable_visibility(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        comment_id: web::Path<Uuid>,
    ) -> AppResponse {
        let service = toggle_comment_visibility_service_factory::exec(&db_conn);

        let user_role = &user.user_role;

        service
            .exec(ToggleCommentVisibilityParams {
                user_role: user_role.as_ref().unwrap(),
                comment_id: comment_id.into_inner(),
            })
            .await?;
        Ok(HttpResponse::NoContent().finish())
    }

    async fn delete(
        db_conn: web::Data<SeaService>,
        comment_id: web::Path<Uuid>,
        user: web::ReqData<ReqUser>,
    ) -> AppResponse {
        let service = delete_comment_service_factory::exec(&db_conn);

        let ReqUser {
            user_role,
            user_id,
            exp: _,
        } = user.into_inner();

        service
            .exec(DeleteCommentParams {
                comment_id: comment_id.into_inner(),
                user_id,
                staff_role: user_role.unwrap(),
            })
            .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
