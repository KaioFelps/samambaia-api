use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::errors::enum_coercion_error::EnumCoercionError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Admin,
    Ceo,
    Coord,
    Editor,
    Principal,
    User,
    Writter,
}

impl FromStr for Role {
    type Err = EnumCoercionError;

    fn from_str(s: &str) -> Result<Self, EnumCoercionError> {
        let s = s.to_uppercase();
        let s = s.as_str();

        match s {
            "ADMIN" => Ok(Self::Admin),
            "CEO" => Ok(Self::Ceo),
            "COORD" => Ok(Self::Coord),
            "EDITOR" => Ok(Self::Editor),
            "PRINCIPAL" => Ok(Self::Principal),
            "USER" => Ok(Self::User),
            "WRITTER" => Ok(Self::Writter),
            _ => Err(EnumCoercionError::new("Role"))
        }
    }
}