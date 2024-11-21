use crate::domain::cryptography::hasher::HasherTrait;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::DomainError;
use crate::util::generate_service_internal_error;

pub struct CreateUserParams {
    pub nickname: String,
    pub password: String,
}
pub struct CreateUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher: Box<dyn HasherTrait>,
}

impl<UserRepositoryType: UserRepositoryTrait> CreateUserService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, hasher: Box<dyn HasherTrait>) -> Self {
        CreateUserService {
            user_repository,
            hasher,
        }
    }

    pub async fn exec(&self, params: CreateUserParams) -> Result<User, DomainError> {
        self.create(params, Role::User).await
    }

    pub async fn exec_with_custom_role(
        &self,
        params: CreateUserParams,
        role: Role,
    ) -> Result<User, DomainError> {
        self.create(params, role).await
    }

    #[inline]
    async fn create(&self, params: CreateUserParams, role: Role) -> Result<User, DomainError> {
        let user_on_db = self
            .user_repository
            .find_by_nickname(&params.nickname)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Create User Service, while finding user by nickname",
                    err,
                )
            })?;

        if let Some(_user) = user_on_db {
            return Err(DomainError::user_already_exists_err(&params.nickname));
        }

        let hashed_password = self.hasher.hash(params.password);

        let user = User::new(params.nickname, hashed_password, Some(role));

        self.user_repository.create(user).await.map_err(|err| {
            generate_service_internal_error(
                "Error occurred on Create User Service, while creating user on database",
                err,
            )
        })
    }
}

#[cfg(test)]
mod test {
    use super::{CreateUserParams, User};
    use crate::domain::{
        cryptography::hasher::MockHasherTrait,
        repositories::user_repository::MockUserRepositoryTrait,
    };

    #[tokio::test]
    async fn test() {
        let mut mocked_repo = MockUserRepositoryTrait::new();

        let mut db: Vec<User> = vec![];

        mocked_repo
            .expect_find_by_nickname()
            .returning(|_| Ok(None));

        mocked_repo
            .expect_create()
            .returning(move |user: User| {
                db.push(user);

                Ok(db[0].clone())
            })
            .times(1);

        let mut mocked_hasher = MockHasherTrait::new();

        mocked_hasher
            .expect_hash()
            .returning(|param_password| format!("{}--hashed", param_password));

        let service = super::CreateUserService {
            user_repository: Box::new(mocked_repo),
            hasher: Box::new(mocked_hasher),
        };

        let result = service
            .exec(CreateUserParams {
                nickname: "Madalena".to_string(),
                password: "madalena123".to_string(),
            })
            .await;

        assert_eq!("Madalena", result.unwrap().nickname());
    }
}
