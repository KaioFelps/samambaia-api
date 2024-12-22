use std::{fmt::Display, io};

#[derive(Debug)]
pub enum SamambaiaCliError {
    ArgumentError(String),
    GeneratorError(String),
}

impl SamambaiaCliError {
    pub fn get_message(&self) -> &str {
        match self {
            SamambaiaCliError::ArgumentError(msg) => msg,
            SamambaiaCliError::GeneratorError(msg) => msg,
        }
    }
}

impl Display for SamambaiaCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

pub trait IntoIoError {
    fn into_io_err(self) -> io::Error;
}

impl IntoIoError for SamambaiaCliError {
    fn into_io_err(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.to_string())
    }
}
