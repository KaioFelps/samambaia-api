use http::StatusCode;
use std::fmt;

use super::error::DomainErrorTrait;

#[derive(Debug, Clone)]
pub struct UserAlreadyExistsError {
    code: u16,
    message: String,
}

impl UserAlreadyExistsError {
    pub fn new(nickname: String) -> Self {
        UserAlreadyExistsError {
            code: StatusCode::CONFLICT.as_u16(),
            message: format!("User with nickname {} already exist.", nickname),
        }
    }
}

impl DomainErrorTrait for UserAlreadyExistsError {
    fn code(&self) -> &u16 {
        &self.code
    }

    fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for UserAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UserAlreadyExistsError {}
