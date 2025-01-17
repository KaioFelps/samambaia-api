use crate::error::SamambaiaError;
use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use serde_json::json;

impl ResponseError for SamambaiaError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.get_code().as_u16()).unwrap()
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let SamambaiaError::Validation(msg, errors_map) = self {
            HttpResponseBuilder::new(self.status_code()).json(json!({
                "code": self.get_code().as_u16(),
                "message": msg,
                "errors": errors_map
            }))
        } else {
            HttpResponseBuilder::new(self.status_code()).json(json!({
                "code": self.get_code().as_u16(),
                "message": self.get_message()
            }))
        }
    }
}
