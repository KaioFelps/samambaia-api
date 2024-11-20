use validator::ValidationErrors;

use crate::error::DomainError;

impl Into<DomainError> for ValidationErrors {
    fn into(self) -> DomainError {
        let errors_map = self.field_errors().to_owned();
        return DomainError::validation_err(&errors_map);
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
