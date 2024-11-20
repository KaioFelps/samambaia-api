use std::fmt;

use http::StatusCode;

use super::error::DomainErrorTrait;

#[derive(Debug, Clone)]
pub struct InvalidCredentialsError {
    code: u16,
    message: String,
}

impl Default for InvalidCredentialsError {
    fn default() -> Self {
        Self::new()
    }
}

impl InvalidCredentialsError {
    pub fn new() -> Self {
        InvalidCredentialsError {
            code: StatusCode::UNAUTHORIZED.as_u16(),
            message: "Invalid credentials.".into(),
        }
    }
}

impl DomainErrorTrait for InvalidCredentialsError {
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
impl fmt::Display for InvalidCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidCredentialsError {}
