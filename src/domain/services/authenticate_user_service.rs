use jsonwebtoken::EncodingKey;
use crate::ENV_VARS;
use crate::infra::jwt::jwt_service::{JwtService, MakeJwtResult};
use crate::errors::{invalid_credentials_error::InvalidCredentialsError, internal_error::InternalError};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use password_auth::verify_password;

pub struct AuthenticateUserParams {
    pub nickname: String,
    pub password: String,
}
pub struct AuthenticateUserService<UserRepository : UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    jwt_service: Box<JwtService>
}

#[derive(Debug)]
pub enum AuthenticateUserServiceErrors<Cred, Internal> {
    InvalidCredentials(Cred),
    InternalError(Internal)
}

impl<UserRepositoryType : UserRepositoryTrait> AuthenticateUserService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, jwt_service: Box<JwtService>) -> Self {
        AuthenticateUserService {
            user_repository,
            jwt_service
        }
    }

    pub async fn exec(&self, params: AuthenticateUserParams) -> Result<MakeJwtResult, AuthenticateUserServiceErrors<InvalidCredentialsError, InternalError>> {
        let user_on_db = &self.user_repository.find_by_nickname(&params.nickname).await;

        if user_on_db.is_err() {
            return Err(AuthenticateUserServiceErrors::InternalError(InternalError::new()));
        }

        let user_on_db = user_on_db.as_ref().unwrap();

        if let None = user_on_db.as_ref() {
            return Err(AuthenticateUserServiceErrors::InvalidCredentials(InvalidCredentialsError::new()));
        }

        let user_on_db = user_on_db.as_ref().unwrap();

        let password_matches = verify_password(&params.password, &user_on_db.password);

        if password_matches.is_err() {
            return Err(AuthenticateUserServiceErrors::InvalidCredentials(InvalidCredentialsError::new()));
        }

        let jwt =
        self.jwt_service
        .make_jwt(
            user_on_db.id,
            user_on_db.role.clone().unwrap(),
            EncodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref())
        );

        match jwt {
            Ok(jwt) => return Ok(jwt),
            Err(_err) => {
                return Err(AuthenticateUserServiceErrors::InternalError(InternalError::new()));
            }
        }

    }
}