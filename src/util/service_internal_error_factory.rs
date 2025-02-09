use std::error::Error;

use log::error;

use crate::error::SamambaiaError;
use crate::{LOG_SEP, R_EOL};

pub fn generate_service_internal_error(message: &str, error: Box<dyn Error>) -> SamambaiaError {
    error!(
        "{R_EOL}{LOG_SEP}{R_EOL}{}: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
        message, error
    );

    SamambaiaError::internal_err()
}
