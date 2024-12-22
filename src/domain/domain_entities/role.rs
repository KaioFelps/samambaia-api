use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::DomainError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Admin,
    Ceo,
    Coord,
    Editor,
    Principal,
    User,
    Writer,
}

impl FromStr for Role {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, DomainError> {
        let s = s.to_uppercase();
        let s = s.as_str();

        match s {
            "ADMIN" => Ok(Self::Admin),
            "CEO" => Ok(Self::Ceo),
            "COORD" => Ok(Self::Coord),
            "EDITOR" => Ok(Self::Editor),
            "PRINCIPAL" => Ok(Self::Principal),
            "USER" => Ok(Self::User),
            "WRITER" => Ok(Self::Writer),
            _ => Err(DomainError::enum_coercion_err("Role")),
        }
    }
}
