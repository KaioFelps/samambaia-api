use actix_web::HttpResponse;

use crate::error::SamambaiaError;

pub type AppResponse<T = HttpResponse> = Result<T, SamambaiaError>;

pub mod announcements_controller;
pub mod article_tags_controller;
pub mod articles_controller;
pub mod comment_reports_controller;
pub mod comments_controller;
pub mod controller;
pub mod free_badges_controller;
pub mod sessions_controller;
pub mod team_roles_controller;
pub mod team_users_controller;
pub mod users_controller;

pub mod admin;
pub mod web;
