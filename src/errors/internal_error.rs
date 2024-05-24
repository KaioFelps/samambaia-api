use std::fmt;

use http::StatusCode;

use super::error::DomainErrorTrait;

#[derive(Debug, Clone)]
pub struct InternalError {
    code: u16,
    message: String,
}

impl InternalError {
    pub fn new() -> Self {
        InternalError {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Internal server error.")
        }
    }
}

impl DomainErrorTrait for InternalError {
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
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InternalError {}