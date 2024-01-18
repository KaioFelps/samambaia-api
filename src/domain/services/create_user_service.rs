use entities::user::Model as UserModel;
use crate::domain::cryptography::hasher::HasherTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::user_already_exists_error::UserAlreadyExistsError;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use entities::sea_orm_active_enums::Role as UserRole;

pub struct CreateUserParams {
    pub nickname: String,
    pub password: String,
}
pub struct CreateUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher: Box<dyn HasherTrait>
}

#[derive(Debug)]
pub enum CreateUserServiceErrors<UserExist, Internal> {
    UserAlreadyExist(UserExist),
    InternalError(Internal)
}

impl<UserRepositoryType : UserRepositoryTrait> CreateUserService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, hasher: Box<dyn HasherTrait>) -> Self {
        CreateUserService {
            user_repository,
            hasher,
        }
    }

    pub async fn exec(&self, params: CreateUserParams)
    -> Result<UserModel, CreateUserServiceErrors<UserAlreadyExistsError, InternalError>>
    { self.create(params, UserRole::User).await }

    pub async fn exec_with_custom_role(&self, params: CreateUserParams, role: UserRole)
    -> Result<UserModel, CreateUserServiceErrors<UserAlreadyExistsError, InternalError>>
    { self.create(params, role).await }

    async fn create(&self, params: CreateUserParams, role: UserRole) -> Result<UserModel, CreateUserServiceErrors<UserAlreadyExistsError, InternalError>> {
        let user_on_db = &self.user_repository.find_by_nickname(&params.nickname).await;

        if user_on_db.is_err() {
            return Err(CreateUserServiceErrors::InternalError(InternalError::new()));
        }

        if let Some(_user) = user_on_db.as_ref().unwrap() {
            return Err(CreateUserServiceErrors::UserAlreadyExist(UserAlreadyExistsError::new(params.nickname)));
        }

        let hashed_password = self.hasher.hash(params.password);

       let created_user = self.user_repository.create(params.nickname, hashed_password, role).await;

        if user_on_db.is_err() {
            return Err(CreateUserServiceErrors::InternalError(InternalError::new()));
        }

        let created_user = created_user.unwrap();

       Ok(created_user)
    }
}