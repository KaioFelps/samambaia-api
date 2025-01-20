use crate::{error::SamambaiaError, LOG_SEP, R_EOL};
use log::error;
use std::error::Error;

pub fn generate_service_internal_error(message: &str, error: Box<dyn Error>) -> SamambaiaError {
    error!(
        "{R_EOL}{LOG_SEP}{R_EOL}{}: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
        message, error
    );

    SamambaiaError::internal_err()
}
