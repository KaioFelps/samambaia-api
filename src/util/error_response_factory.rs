use crate::{errors::error::DomainErrorTrait, infra::http::presenters::error::ErrorPresenter};
use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};

pub fn generate_error_response(err: Box<dyn DomainErrorTrait>) -> HttpResponse {
    return HttpResponseBuilder::new(StatusCode::from_u16(err.code().to_owned()).unwrap())
        .json(ErrorPresenter::to_http(err));
}
