use actix_session::Session;
use actix_web::web::{self, Data, Json, Path, Redirect};
use actix_web::HttpRequest;
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade};
use uuid::Uuid;

use crate::domain::factories::journalism::comments::comment_on_article_service_factory;
use crate::domain::services::journalism::comments::comment_on_article_service::CommentOnArticleParams;
use crate::infra::extensions::sessions::SessionHelpers;
use crate::infra::http::controllers::AppResponse;
use crate::infra::http::dtos::comment_on_article::CommentOnArticleDto;
use crate::infra::http::middlewares::web::WebAuthUser;
use crate::infra::http::middlewares::WebAuthUserMiddleware;
use crate::infra::http::routes::route::RouteTrait;
use crate::infra::sea::sea_service::SeaService;

pub struct CommentsController;

impl RouteTrait for CommentsController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("comments")
                .wrap(WebAuthUserMiddleware)
                .route("{article_id}/new", web::post().to(Self::comment_on_article)),
        );
    }
}

impl CommentsController {
    pub async fn comment_on_article(
        req: HttpRequest,
        auth_user: WebAuthUser,
        article_id: Path<Uuid>,
        body: Json<CommentOnArticleDto>,
        db_conn: Data<SeaService>,
    ) -> AppResponse<Redirect> {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect_back) => return Ok(redirect_back),
        };

        let service = comment_on_article_service_factory::exec(&db_conn);

        let _ = service
            .exec(CommentOnArticleParams {
                article_id: article_id.into_inner(),
                author_id: auth_user.user.id(),
                content: body.content.unwrap(),
            })
            .await?;

        Session::flash_silently(&req, "commentSuccess", "Comentário registrado com sucesso!");

        Ok(Inertia::back(&req))
    }
}
