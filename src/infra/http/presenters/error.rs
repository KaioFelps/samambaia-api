use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use validator::ValidationError;

use crate::errors::error::DomainErrorTrait;

#[derive(Serialize, Deserialize)]
pub struct MappedError {
    code: u16,
    message: String,
}

pub struct ErrorPresenter;

impl ErrorPresenter {
    pub fn to_http(error: Box<dyn DomainErrorTrait>) -> MappedError {
        MappedError {
            code: error.code().to_owned(),
            message: error.message().to_owned(),
        }
    }

    pub fn to_http_from_validator(
        errors: HashMap<&'static str, &Vec<ValidationError>>,
    ) -> JsonValue {
        json!({
            "code": StatusCode::BAD_REQUEST.as_u16(),
            "errors": errors
        })
    }
}
