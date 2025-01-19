use validator::ValidationErrors;

use crate::error::{IntoSamambaiaError, SamambaiaError};

impl From<ValidationErrors> for SamambaiaError {
    fn from(value: ValidationErrors) -> Self {
        let errors_map = value.field_errors().to_owned();
        SamambaiaError::validation_err(&errors_map)
    }
}

impl IntoSamambaiaError for ValidationErrors {
    fn into_samambaia_error(self) -> SamambaiaError {
        self.into()
    }
}
