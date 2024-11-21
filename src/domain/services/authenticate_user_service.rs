use crate::domain::cryptography::comparer::ComparerTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::DomainError;
use crate::infra::jwt::jwt_service::{JwtService, MakeJwtResult};
use crate::util::generate_service_internal_error;
use crate::ENV_VARS;

use jsonwebtoken::EncodingKey;

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

    pub async fn exec(&self, params: AuthenticateUserParams) -> Result<MakeJwtResult, DomainError> {
        let user_on_db = self
            .user_repository
            .find_by_nickname(&params.nickname)
            .await
            .map_err(|err|
                generate_service_internal_error("Error occurred on Authenticate User Service, while fetching user from database",
                err)
            )?;

        if user_on_db.is_none() {
            return Err(DomainError::invalid_credentials_err());
        }

        let user_on_db = user_on_db.unwrap();

        let password_matches = self
            .comparer
            .compare(&params.password, user_on_db.password());

        if !password_matches {
            return Err(DomainError::invalid_credentials_err());
        }

        let jwt = self.jwt_service.make_jwt(
            user_on_db.id(),
            user_on_db.role().unwrap(),
            EncodingKey::from_secret(ENV_VARS.jwt_secret.as_ref()),
        );

        match jwt {
            Ok(jwt) => Ok(jwt),
            Err(_err) => Err(DomainError::internal_err()),
        }
    }
}
