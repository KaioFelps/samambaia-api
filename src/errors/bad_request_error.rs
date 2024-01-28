use std::fmt;
use http::StatusCode;

#[derive(Debug, Clone)]
pub struct BadRequestError {
    code: u16,
    message: String,
}

impl BadRequestError {
    pub fn new() -> Self {
        BadRequestError {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: "Bad request performed.".to_string()
        }
    }

    pub fn code(&self) -> &u16 {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BadRequestError {}