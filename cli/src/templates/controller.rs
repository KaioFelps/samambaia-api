pub fn get_controller_template(capitalized_entity: &String) -> String {
    let lower_case_entity = capitalized_entity.clone().to_lowercase();

    format!(
        r#"use actix_web::{{web, HttpResponse, Responder}};
use super::controller::ControllerTrait;
use super::AppResponse;

pub struct {capitalized_entity}sController;

impl ControllerTrait for {capitalized_entity}sController {{
    fn register(cfg: &mut web::ServiceConfig) {{
        cfg.service(web::scope("/{lower_case_entity}s")
            // CREATE
            .route("/new", web::post().to(Self::create))

            // READ
            .route("/{{id}}/get", web::get().to(Self::get))
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{{id}}/update", web::put().to(Self::update))

            // DELETE
            .route("/{{id}}/delete", web::delete().to(Self::delete))
        );
    }}
}}

impl {capitalized_entity}sController {{
    async fn create() -> AppResponse {{
        // let service = service_factory::exec()?;
        // service.exec()?;

        Ok(HttpResponse::Created().finish())
    }}

    async fn get() -> AppResponse {{
        Ok(HttpResponse::Ok().finish())
    }}

    async fn list() -> AppResponse {{
        Ok(HttpResponse::Ok().finish())
    }}

    async fn update() -> AppResponse {{
        Ok(HttpResponse::NoContent().finish())
    }}

    async fn delete() -> AppResponse {{
        Ok(HttpResponse::NoContent().finish())
    }}
}}
"#
    )
}
