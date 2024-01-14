use http::StatusCode;

#[derive(Debug)]
pub struct DatabaseError {
    code: u16,
    message: String,
}

impl DatabaseError {
    pub fn new() -> Self {
        DatabaseError {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Internal server error.")
        }
    }

    pub fn code(&self) -> &u16 {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}