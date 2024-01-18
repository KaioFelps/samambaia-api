use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Admin,
    Ceo,
    Coord,
    Editor,
    Principal,
    User,
    Writter,
}
