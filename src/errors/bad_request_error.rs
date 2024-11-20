use http::StatusCode;
use std::fmt;

use super::error::DomainErrorTrait;

#[derive(Debug, Clone)]
pub struct BadRequestError {
    code: u16,
    message: String,
}

impl Default for BadRequestError {
    fn default() -> Self {
        Self::new()
    }
}

impl BadRequestError {
    pub fn new() -> Self {
        BadRequestError {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: "Bad request performed.".to_string(),
        }
    }

    pub fn new_with_message(message: String) -> Self {
        BadRequestError {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message,
        }
    }
}

impl DomainErrorTrait for BadRequestError {
    fn code(&self) -> &u16 {
        &self.code
    }

    fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BadRequestError {}
