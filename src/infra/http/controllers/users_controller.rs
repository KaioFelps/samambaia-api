use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::factories::update_user_service_factory;
use crate::domain::services::update_user_service::UpdateUserParams;
use crate::domain::domain_entities::role::Role;
use crate::infra::http::middlewares::authentication_middleware;
use crate::infra::jwt::jwt_service::DecodedToken;

pub struct UsersController {}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>
}

impl UsersController {
    pub fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/users")
            .route("/new", web::post().to(Self::create))
            .route("/{id}/update", web::put().to(Self::update).wrap(from_fn(authentication_middleware)))
        );
    }

    async fn create() -> impl Responder {
        HttpResponse::Ok().body("Hello World")
    }

    async fn update(
        path: web::Path<Uuid>,
        body: web::Json<UpdateUser>,
        token: Option<web::ReqData<DecodedToken>>,
    ) -> impl Responder {
        let user_id: Uuid = path.into_inner();
    
        let UpdateUser {nickname, password, role} = body.into_inner();
    
        if let None = token {
            return HttpResponse::Unauthorized();
        }

        let token: DecodedToken = token.unwrap().into_inner();

        let update_user = update_user_service_factory::exec().await;
        let _res = update_user.exec(UpdateUserParams {
            nickname,
            password,
            role,
            user_id,
            staff_id: token.user_id,
            staff_role: token.user_role.unwrap()
        }).await;
    
        HttpResponse::NoContent()
    }
}
