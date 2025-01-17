use validator::ValidationErrors;

use crate::error::SamambaiaError;

impl From<ValidationErrors> for SamambaiaError {
    fn from(value: ValidationErrors) -> Self {
        let errors_map = value.field_errors().to_owned();
        SamambaiaError::validation_err(&errors_map)
    }
}

pub trait IntoSamambaiaError {
    fn into_domain_err(self) -> SamambaiaError;
}

impl IntoSamambaiaError for ValidationErrors {
    fn into_domain_err(self) -> SamambaiaError {
        self.into()
    }
}
