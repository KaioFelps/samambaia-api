use actix_web::middleware::from_fn;
use actix_web::web::Query;
use actix_web::{HttpResponse, web};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use super::AppResponse;
use super::controller::ControllerTrait;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::factories::identity::get_user_service_factory;
use crate::domain::factories::journalism::articles::{
    create_article_service_factory,
    delete_article_service_factory,
    fetch_many_articles_service_factory,
    get_expanded_article_service_factory,
    update_article_service_factory,
};
use crate::domain::services::identity::get_user_service::GetUserServiceParams;
use crate::domain::services::journalism::articles::create_article_service::CreateArticleParams;
use crate::domain::services::journalism::articles::delete_article_service::DeleteArticleParams;
use crate::domain::services::journalism::articles::fetch_articles_services::{
    FetchArticleQuery,
    FetchArticlesParams,
};
use crate::domain::services::journalism::articles::get_expanded_article_service::{
    FetchManyCommentsWithAuthorResponse,
    GetExpandedArticleParams,
    GetExpandedArticleResponse,
};
use crate::domain::services::journalism::articles::update_article_service::UpdateArticleParams;
use crate::domain::value_objects::slug::Slug;
use crate::error::{IntoSamambaiaError, SamambaiaError};
use crate::infra::http::dtos::controllers::articles::ShowArticleQueryDto;
use crate::infra::http::dtos::create_article::CreateArticleDto;
use crate::infra::http::dtos::list_article_admin::AdminListArticlesDto;
use crate::infra::http::dtos::list_articles::ListArticlesDto;
use crate::infra::http::dtos::update_article::UpdateArticleDto;
use crate::infra::http::extractors::req_user::ReqUser;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::http::presenters::article::{ArticlePresenter, MappedArticle};
use crate::infra::http::presenters::expanded_article::ExpandedArticlePresenter;
use crate::infra::http::presenters::pagination::PaginationPresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::sea::sea_service::SeaService;

pub struct ArticlesController;

impl ControllerTrait for ArticlesController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/articles")
                // CREATE
                .route(
                    "/new",
                    web::post()
                        .to(Self::create)
                        .wrap(from_fn(authentication_middleware)),
                )
                // READ
                .route("/{slug}/get", web::get().to(Self::get))
                .route("/list", web::get().to(Self::list))
                .route(
                    "/list/admin",
                    web::get()
                        .to(Self::admin_list)
                        .wrap(from_fn(authentication_middleware)),
                )
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

impl ArticlesController {
    async fn create(
        db_conn: web::Data<SeaService>,
        body: web::Json<CreateArticleDto>,
        user: web::ReqData<ReqUser>,
    ) -> AppResponse {
        body.validate().map_err(|err| err.into_samambaia_error())?;

        let body = body.into_inner();
        let auth_user = user.into_inner();

        let find_user_service = get_user_service_factory::exec(&db_conn);

        let user = match find_user_service
            .exec(GetUserServiceParams {
                user_id: auth_user.user_id,
            })
            .await?
        {
            Some(user) => user,
            None => return Err(SamambaiaError::resource_not_found_err()),
        };

        let service = create_article_service_factory::exec(&db_conn);

        let CreateArticleDto {
            author_id,
            content,
            cover_url,
            title,
            description,
            tags,
            script,
        } = body;

        let article = service
            .exec(CreateArticleParams {
                custom_author_id: author_id,
                content,
                cover_url,
                title,
                description,
                staff: &user,
                tags,
                script,
            })
            .await?;

        let mapped_article = ArticlePresenter::to_http(article);

        Ok(HttpResponse::Created().json(json!({"data": mapped_article})))
    }

    async fn get(
        db_conn: web::Data<SeaService>,
        query: Query<ShowArticleQueryDto>,
        article_slug: web::Path<String>,
        user: Option<web::ReqData<ReqUser>>,
    ) -> AppResponse {
        let service = get_expanded_article_service_factory::exec(&db_conn);

        let (user_id, user_role) = match &user {
            None => (None, None),
            Some(user) => (Some(&user.user_id), Some(user.user_role.as_ref().unwrap())),
        };

        let GetExpandedArticleResponse {
            article,
            article_author,
            comments: comment_response,
        } = service
            .exec(GetExpandedArticleParams {
                article_slug: Slug::new_from_existing(article_slug.into_inner()),
                comments_per_page: Some(DEFAULT_PER_PAGE as u32),
                comments_page: query.comments_page,
                user_id,
                user_role,
            })
            .await?;

        let FetchManyCommentsWithAuthorResponse {
            data: comments,
            pagination: comments_pagination,
        } = comment_response;

        let mapped_article = ExpandedArticlePresenter::to_http(
            article,
            article_author,
            comments,
            (comments_pagination, DEFAULT_PER_PAGE),
        );

        Ok(HttpResponse::Ok().json(json!({
            "data": mapped_article,
        })))
    }

    async fn list(
        db_conn: web::Data<SeaService>,
        query: web::Query<ListArticlesDto>,
    ) -> AppResponse {
        let query_body = query
            .validate()
            .map_err(|err| err.into_samambaia_error())
            .map(|_| query.into_inner())?;

        Self::get_list_of_articles(
            &db_conn,
            query_body.title,
            query_body.author,
            query_body.page,
            query_body.per_page,
            Some(true),
        )
        .await
    }

    async fn admin_list(
        db_conn: web::Data<SeaService>,
        query: web::Query<AdminListArticlesDto>,
    ) -> AppResponse {
        let query_body = query
            .validate()
            .map(|_| query.into_inner())
            .map_err(|err| err.into_samambaia_error())?;

        Self::get_list_of_articles(
            &db_conn,
            query_body.title,
            query_body.author,
            query_body.page,
            query_body.per_page,
            query_body.approved_state,
        )
        .await
    }

    async fn update(
        db_conn: web::Data<SeaService>,
        user: web::ReqData<ReqUser>,
        body: web::Json<UpdateArticleDto>,
        article_id: web::Path<Uuid>,
    ) -> AppResponse {
        let UpdateArticleDto {
            title,
            approved,
            cover_url,
            content,
            author_id,
            description,
            tags,
            script,
        } = body
            .validate()
            .map(|_| body.into_inner())
            .map_err(|err| err.into_samambaia_error())?;

        let service = update_article_service_factory::exec(&db_conn);

        let ReqUser { user_id, .. } = user.into_inner();

        let find_user_service = get_user_service_factory::exec(&db_conn);

        let user = match find_user_service
            .exec(GetUserServiceParams { user_id })
            .await?
        {
            Some(user) => user,
            None => return Err(SamambaiaError::unauthorized_err()),
        };

        let article = service
            .exec(UpdateArticleParams {
                user: &user,
                content,
                cover_url,
                approved,
                article_id: article_id.into_inner(),
                title,
                author_id,
                description,
                tags,
                script,
            })
            .await?;

        let mapped_article = ArticlePresenter::to_http(article);

        Ok(HttpResponse::Ok().json(json!({"data": mapped_article})))
    }

    async fn delete(
        db_conn: web::Data<SeaService>,
        req_user: web::ReqData<ReqUser>,
        article_id: web::Path<Uuid>,
    ) -> AppResponse {
        let find_user_service = get_user_service_factory::exec(&db_conn);

        let user = match find_user_service
            .exec(GetUserServiceParams {
                user_id: req_user.user_id,
            })
            .await?
        {
            Some(user) => user,
            None => return Err(SamambaiaError::unauthorized_err()),
        };

        let service = delete_article_service_factory::exec(&db_conn);

        service
            .exec(DeleteArticleParams {
                user: &user,
                article_id: article_id.into_inner(),
            })
            .await
            .map(|_| HttpResponse::NoContent().finish())
    }

    async fn get_list_of_articles(
        db_conn: &SeaService,
        title: Option<String>,
        author: Option<String>,
        page: Option<u32>,
        per_page: Option<u8>,
        approved_state: Option<bool>,
    ) -> AppResponse {
        let service = fetch_many_articles_service_factory::exec(db_conn);

        let query = {
            if let Some(title) = title {
                Some(FetchArticleQuery::Title(title))
            } else {
                author.map(FetchArticleQuery::Author)
            }
        };

        service
            .exec(FetchArticlesParams {
                page,
                per_page: per_page.map(|pp| pp as u32),
                query,
                approved_state,
            })
            .await
            .map(|articles| {
                let mapped_articles = articles
                .data
                .into_iter()
                .map(ArticlePresenter::to_http)
                .collect::<Vec<MappedArticle>>();

                HttpResponse::Ok().json(json!({
                    "pagination": PaginationPresenter::to_http(articles.pagination, per_page.unwrap_or(DEFAULT_PER_PAGE)),
                    "data": mapped_articles
                }))
            })
    }
}
