use std::collections::HashMap;

use actix_web::web;
use actix_web::web::Redirect;

use crate::error::SamambaiaError;
use crate::infra::http::controllers::controller::ControllerTrait;
use crate::infra::imager::factory::ImagerFactory;

pub struct ImagerController;

impl ControllerTrait for ImagerController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.route("imager", web::get().to(Self::get_image));
    }
}

impl ImagerController {
    async fn get_image(
        query: web::Query<HashMap<String, String>>,
    ) -> Result<Redirect, SamambaiaError> {
        let mut params = query.into_inner();
        let nickname = match params.remove("user") {
            None => {
                return Err(SamambaiaError::bad_request_err()
                    .with_message("Missing `user` query parameter."))
            }
            Some(nickname) => nickname,
        };

        let imager_url = ImagerFactory::get_imager()
            .mount_imager_url(&nickname, params)
            .await?;

        Ok(Redirect::to(imager_url).see_other())

        // let image = reqwest::get(&imager_url).await.map_err(|err| {
        //     generate_service_internal_error(
        //         "Failed to fetch user avatar on final imager url",
        //         Box::new(err),
        //     )
        // })?;

        // let content_length = image.content_length();

        // let image_bytes = image
        //     .bytes()
        //     .await
        //     .map_err(|err| {
        //         generate_service_internal_error(
        //             "Failed to get bytes from avatar image response in `ImagerController::get_image` handler",
        //             Box::new(err)
        //         )
        //     })?;

        // Ok(HttpResponse::Ok()
        //     .content_type(ContentType::png())
        //     .insert_header(header::ContentLength(
        //         content_length.unwrap_or(image_bytes.len() as u64) as usize,
        //     ))
        //     .body(image_bytes))
    }
}
