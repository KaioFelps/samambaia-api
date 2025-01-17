use crate::configs::app::APP_CONFIG;
use crate::domain::cryptography::comparer::ComparerTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::infra::jwt::jwt_service::{JwtService, MakeJwtResult};
use crate::util::generate_service_internal_error;

use jsonwebtoken::EncodingKey;

pub struct AuthenticateUserParams {
    pub nickname: String,
    pub password: String,
}
pub struct AuthenticateUserService<UserRepository: UserRepositoryTrait, Comparer: ComparerTrait> {
    user_repository: UserRepository,
    jwt_service: JwtService,
    comparer: Comparer,
}

impl<UserRepositoryType: UserRepositoryTrait, Comparer: ComparerTrait>
    AuthenticateUserService<UserRepositoryType, Comparer>
{
    pub fn new(
        user_repository: UserRepositoryType,
        jwt_service: JwtService,
        comparer: Comparer,
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
    ) -> Result<MakeJwtResult, SamambaiaError> {
        let user_on_db = self
            .user_repository
            .find_by_nickname(&params.nickname)
            .await
            .map_err(|err|
                generate_service_internal_error("Error occurred on Authenticate User Service, while fetching user from database",
                err)
            )?;

        if user_on_db.is_none() {
            return Err(SamambaiaError::invalid_credentials_err());
        }

        let user_on_db = user_on_db.unwrap();

        let password_matches = self
            .comparer
            .compare(&params.password, user_on_db.password());

        if !password_matches {
            return Err(SamambaiaError::invalid_credentials_err());
        }

        let jwt = self.jwt_service.make_jwt(
            user_on_db.id(),
            user_on_db.role().unwrap(),
            EncodingKey::from_secret(APP_CONFIG.jwt_secret.as_ref()),
        );

        match jwt {
            Ok(jwt) => Ok(jwt),
            Err(_err) => Err(SamambaiaError::internal_err()),
        }
    }
}
