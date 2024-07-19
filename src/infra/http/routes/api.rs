use actix_web::web;

use crate::infra::http::controllers::articles_controller::ArticlesController;
use crate::infra::http::controllers::comment_reports_controller::CommentReportsController;
use crate::infra::http::controllers::comments_controller::CommentsController;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::http::controllers::sessions_controller::SessionsController;
use crate::infra::http::controllers::team_roles_controller::TeamRolesController;
use crate::infra::http::controllers::team_users_controller::TeamUsersController;
use crate::infra::http::middlewares::RequestUserMiddleware;
use crate::infra::http::controllers::users_controller::UsersController;
use crate::infra::http::routes::route::RouteTrait;

pub struct ApiRoutes;

impl RouteTrait for ApiRoutes {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
        web::scope("api")
            .wrap(RequestUserMiddleware)
            .configure(UsersController::register)
            .configure(SessionsController::register)
            .configure(ArticlesController::register)
            .configure(CommentsController::register)
            .configure(CommentReportsController::register)
            .configure(TeamRolesController::register)
            .configure(TeamUsersController::register)
        );
    }
}
