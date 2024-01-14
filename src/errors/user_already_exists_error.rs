use http::StatusCode;

#[derive(Debug)]
pub struct UserAlreadyExistsError {
    code: u16,
    message: String,
}

impl UserAlreadyExistsError {
    pub fn new(nickname: String) -> Self {
        UserAlreadyExistsError {
            code: StatusCode::CONFLICT.as_u16(),
            message: format!("User with nickname {} already exist.", nickname)
        }
    }

    pub fn code(&self) -> &u16 {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}