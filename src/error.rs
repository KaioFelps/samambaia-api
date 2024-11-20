use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use http::StatusCode;
use validator::ValidationError;

pub trait DomainErrorTrait: Error {
    fn code(&self) -> &u16;
    fn message(&self) -> &String;
}

type ErrorsMap = HashMap<String, Vec<ValidationError>>;

#[derive(Debug, Clone)]
pub enum DomainError {
    BadRequest(String),
    EnumCoercion(String),
    Internal(String),
    InvalidCredentials(String),
    ResourceNotFound(String),
    Unauthorized(String),
    UserAlreadyExists(String),
    Validation(String, ErrorsMap),
}

impl DomainError {
    pub fn get_code(&self) -> StatusCode {
        match self {
            DomainError::BadRequest(_) => StatusCode::BAD_REQUEST,
            DomainError::EnumCoercion(_) => StatusCode::BAD_REQUEST,
            DomainError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DomainError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            DomainError::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            DomainError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            DomainError::Validation(_, _) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            DomainError::BadRequest(msg)
            | DomainError::EnumCoercion(msg)
            | DomainError::Internal(msg)
            | DomainError::InvalidCredentials(msg)
            | DomainError::ResourceNotFound(msg)
            | DomainError::Unauthorized(msg)
            | DomainError::UserAlreadyExists(msg)
            | DomainError::Validation(msg, _) => &msg,
        }
    }

    pub fn with_message<T>(self, msg: T) -> Self
    where
        T: ToString,
    {
        let msg = msg.to_string();

        match self {
            DomainError::BadRequest(_) => DomainError::BadRequest(msg),
            DomainError::EnumCoercion(_) => DomainError::EnumCoercion(msg),
            DomainError::Internal(_) => DomainError::Internal(msg),
            DomainError::InvalidCredentials(_) => DomainError::InvalidCredentials(msg),
            DomainError::ResourceNotFound(_) => DomainError::ResourceNotFound(msg),
            DomainError::Unauthorized(_) => DomainError::Unauthorized(msg),
            DomainError::UserAlreadyExists(_) => DomainError::UserAlreadyExists(msg),
            DomainError::Validation(_, errors_map) => DomainError::Validation(msg, errors_map),
        }
    }

    pub fn unauthorized_err() -> Self {
        Self::Unauthorized("Unauthorized.".into())
    }

    pub fn bad_request_err() -> Self {
        Self::BadRequest("Bad request performed.".into())
    }

    pub fn internal_err() -> Self {
        Self::Internal("Internal server error.".into())
    }

    pub fn invalid_credentials_err() -> Self {
        Self::InvalidCredentials("Invalid credentials.".into())
    }

    pub fn resource_not_found_err() -> Self {
        DomainError::ResourceNotFound("Resource not found.".into())
    }

    pub fn user_already_exists_err(nickname: &str) -> Self {
        Self::UserAlreadyExists(format!("User with nickname {} already exist.", nickname))
    }

    pub fn validation_err<'b>(
        errors_map: &'b HashMap<&'b str, &Vec<validator::ValidationError>>,
    ) -> Self {
        let mut _errors_map = HashMap::new();

        errors_map.iter().map(|(k, v)| {
            let mut vec = Vec::new();
            v.iter().map(|v| {
                vec.push(v.clone());
            });

            _errors_map.insert(k.to_string(), vec);
        });

        Self::Validation("Validation errors.".into(), _errors_map)
    }

    pub fn enum_coercion_err(enum_name: &str) -> Self {
        Self::EnumCoercion(format!("{enum_name} enum coercion error.").into())
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for DomainError {}
