use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use http::StatusCode;
use validator::ValidationError;

pub trait SamambaiaErrorTrait: Error {
    fn code(&self) -> &u16;
    fn message(&self) -> &String;
}

pub trait IntoSamambaiaError {
    fn into_samambaia_error(self) -> SamambaiaError;
}

type ErrorsMap = HashMap<String, Vec<ValidationError>>;

#[derive(Debug, Clone)]
pub enum SamambaiaError {
    BadRequest(String),
    EnumCoercion(String),
    Internal(String),
    InvalidCredentials(String),
    ResourceNotFound(String),
    Unauthorized(String),
    UserAlreadyExists(String),
    Validation(String, ErrorsMap),
}

impl SamambaiaError {
    pub fn get_code(&self) -> StatusCode {
        match self {
            SamambaiaError::BadRequest(_) => StatusCode::BAD_REQUEST,
            SamambaiaError::EnumCoercion(_) => StatusCode::BAD_REQUEST,
            SamambaiaError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SamambaiaError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            SamambaiaError::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            SamambaiaError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            SamambaiaError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            SamambaiaError::Validation(_, _) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            SamambaiaError::BadRequest(msg)
            | SamambaiaError::EnumCoercion(msg)
            | SamambaiaError::Internal(msg)
            | SamambaiaError::InvalidCredentials(msg)
            | SamambaiaError::ResourceNotFound(msg)
            | SamambaiaError::Unauthorized(msg)
            | SamambaiaError::UserAlreadyExists(msg)
            | SamambaiaError::Validation(msg, _) => msg,
        }
    }

    pub fn with_message<T>(self, msg: T) -> Self
    where
        T: ToString,
    {
        let msg = msg.to_string();

        match self {
            SamambaiaError::BadRequest(_) => SamambaiaError::BadRequest(msg),
            SamambaiaError::EnumCoercion(_) => SamambaiaError::EnumCoercion(msg),
            SamambaiaError::Internal(_) => SamambaiaError::Internal(msg),
            SamambaiaError::InvalidCredentials(_) => SamambaiaError::InvalidCredentials(msg),
            SamambaiaError::ResourceNotFound(_) => SamambaiaError::ResourceNotFound(msg),
            SamambaiaError::Unauthorized(_) => SamambaiaError::Unauthorized(msg),
            SamambaiaError::UserAlreadyExists(_) => SamambaiaError::UserAlreadyExists(msg),
            SamambaiaError::Validation(_, errors_map) => {
                SamambaiaError::Validation(msg, errors_map)
            }
        }
    }

    pub fn unauthorized_err() -> Self {
        Self::Unauthorized("Não autorizado.".into())
    }

    pub fn bad_request_err() -> Self {
        Self::BadRequest("Request mal formada.".into())
    }

    pub fn internal_err() -> Self {
        Self::Internal("Erro interno do servidor.".into())
    }

    pub fn invalid_credentials_err() -> Self {
        Self::InvalidCredentials("Credenciais inválidas.".into())
    }

    pub fn resource_not_found_err() -> Self {
        SamambaiaError::ResourceNotFound("Recurso não encontrado.".into())
    }

    pub fn user_already_exists_err(nickname: &str) -> Self {
        Self::UserAlreadyExists(format!("Já existe um user com o apelido '{}'.", nickname))
    }

    pub fn validation_err<'b>(
        errors_map: &'b HashMap<&'b str, &Vec<validator::ValidationError>>,
    ) -> Self {
        let mut _errors_map = errors_map
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_vec()))
            .collect();

        Self::Validation("Erros de validação.".into(), _errors_map)
    }

    pub fn enum_coercion_err(enum_name: &str) -> Self {
        Self::EnumCoercion(format!("{enum_name} enum coercion error."))
    }
}

impl Display for SamambaiaError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for SamambaiaError {}
