use std::{fmt::Display, io};

#[derive(Debug)]
pub enum HubbitosCliError {
    ArgumentError(String),
    GeneratorError(String),
}

impl HubbitosCliError {
    pub fn get_message(&self) -> &str {
        match self {
            HubbitosCliError::ArgumentError(msg) => msg,
            HubbitosCliError::GeneratorError(msg) => msg,
        }
    }
}

impl Display for HubbitosCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

pub trait IntoIoError {
    fn into_io_err(self) -> io::Error;
}

impl IntoIoError for HubbitosCliError {
    fn into_io_err(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.to_string())
    }
}
