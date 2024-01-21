use serde::{Serialize, Deserialize};

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
