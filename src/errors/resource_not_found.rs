use std::fmt;

use http::StatusCode;

use super::error::DomainErrorTrait;

#[derive(Debug, Clone)]
pub struct ResourceNotFoundError {
    code: u16,
    message: String,
}

impl Default for ResourceNotFoundError {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceNotFoundError {
    pub fn new() -> Self {
        ResourceNotFoundError {
            code: StatusCode::NOT_FOUND.as_u16(),
            message: "Resource not found.".into(),
        }
    }
}

impl DomainErrorTrait for ResourceNotFoundError {
    fn code(&self) -> &u16 {
        &self.code
    }

    fn message(&self) -> &String {
        &self.message
    }
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for ResourceNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ResourceNotFoundError {}
