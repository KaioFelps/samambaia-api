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
    fn from_str(s: &str) -> Result<Self, EnumCoercionError> {
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
    
    type Err = EnumCoercionError;
}