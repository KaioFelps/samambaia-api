use validator::ValidationErrors;

use crate::error::DomainError;

impl From<ValidationErrors> for DomainError {
    fn from(value: ValidationErrors) -> Self {
        let errors_map = value.field_errors().to_owned();
        DomainError::validation_err(&errors_map)
    }
}

pub trait IntoDomainError {
    fn into_domain_err(self) -> DomainError;
}

impl IntoDomainError for ValidationErrors {
    fn into_domain_err(self) -> DomainError {
        self.into()
    }
}
