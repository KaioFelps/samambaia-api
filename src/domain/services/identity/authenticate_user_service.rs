use crate::domain::cryptography::comparer::ComparerTrait;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub struct AuthenticateUserParams {
    pub nickname: String,
    pub password: String,
}
pub struct AuthenticateUserService<UserRepository: UserRepositoryTrait, Comparer: ComparerTrait> {
    user_repository: UserRepository,
    comparer: Comparer,
}

impl<UserRepositoryType: UserRepositoryTrait, Comparer: ComparerTrait>
    AuthenticateUserService<UserRepositoryType, Comparer>
{
    pub fn new(user_repository: UserRepositoryType, comparer: Comparer) -> Self {
        AuthenticateUserService {
            user_repository,
            comparer,
        }
    }

    pub async fn exec(&self, params: AuthenticateUserParams) -> Result<User, SamambaiaError> {
        let user_on_db = self
            .user_repository
            .find_by_nickname(&params.nickname)
            .await
            .map_err(|err|
                generate_service_internal_error("Error occurred on Authenticate User Service, while fetching user from database",
                err)
            )?;

        if let Some(user) = user_on_db {
            let password_matches = self.comparer.compare(&params.password, user.password());

            if password_matches {
                return Ok(user);
            }
        }

        Err(SamambaiaError::invalid_credentials_err())
    }
}
