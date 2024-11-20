use jsonwebtoken::EncodingKey;
use log::error;

use crate::domain::cryptography::comparer::ComparerTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::{
    internal_error::InternalError, invalid_credentials_error::InvalidCredentialsError,
};
use crate::infra::jwt::jwt_service::{JwtService, MakeJwtResult};
use crate::ENV_VARS;

use crate::{LOG_SEP, R_EOL};

pub struct AuthenticateUserParams {
    pub nickname: String,
    pub password: String,
}
pub struct AuthenticateUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    jwt_service: Box<JwtService>,
    comparer: Box<dyn ComparerTrait>,
}

impl<UserRepositoryType: UserRepositoryTrait> AuthenticateUserService<UserRepositoryType> {
    pub fn new(
        user_repository: Box<UserRepositoryType>,
        jwt_service: Box<JwtService>,
        comparer: Box<dyn ComparerTrait>,
    ) -> Self {
        AuthenticateUserService {
            user_repository,
            jwt_service,
            comparer,
        }
    }

    pub async fn exec(
        &self,
        params: AuthenticateUserParams,
    ) -> Result<MakeJwtResult, Box<dyn DomainErrorTrait>> {
        let user_on_db = &self
            .user_repository
            .find_by_nickname(&params.nickname)
            .await;

        if user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Authenticate User Service, while fetching user from database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let user_on_db = user_on_db.as_ref().unwrap();

        if user_on_db.as_ref().is_none() {
            return Err(Box::new(InvalidCredentialsError::new()));
        }

        let user_on_db = user_on_db.as_ref().unwrap();

        let password_matches = self
            .comparer
            .compare(&params.password, user_on_db.password());

        if !password_matches {
            return Err(Box::new(InvalidCredentialsError::new()));
        }

        let jwt = self.jwt_service.make_jwt(
            user_on_db.id(),
            user_on_db.role().unwrap(),
            EncodingKey::from_secret(ENV_VARS.jwt_secret.as_ref()),
        );

        match jwt {
            Ok(jwt) => Ok(jwt),
            Err(_err) => Err(Box::new(InternalError::new())),
        }
    }
}
