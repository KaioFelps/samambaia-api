pub fn get_controller_template(
    capitalized_entity: &String,
) -> String {
    let lower_case_entity = capitalized_entity.clone().to_lowercase();

    return format!(
r#"use actix_web::{{web, HttpResponse, Responder}};

use super::controller::ControllerTrait;

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
    async fn create() -> impl Responder {{
        // let service = service_factory::exec().await;
        //
        // let result = service.exec();
        //
        // if result.is_err() {{
        //     let err = result.unwrap_err();
        //
        //     return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
        //     .json(ErrorPresenter::to_http(err));
        // }}

        return HttpResponse::Created().finish();
    }}

    async fn get() -> impl Responder {{
        return HttpResponse::Ok().finish();
    }}

    async fn list() -> impl Responder {{
        return HttpResponse::Ok().finish();
    }}

    async fn update() -> impl Responder {{
        return HttpResponse::NoContent().finish();
    }}

    async fn delete() -> impl Responder {{
        return HttpResponse::NoContent().finish();
    }}
}}
"#
    );
}
