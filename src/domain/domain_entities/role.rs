use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::SamambaiaError;

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
    type Err = SamambaiaError;

    fn from_str(s: &str) -> Result<Self, SamambaiaError> {
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
            _ => Err(SamambaiaError::enum_coercion_err("Role")),
        }
    }
}
