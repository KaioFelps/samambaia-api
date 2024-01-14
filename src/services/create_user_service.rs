use entities::user::Model as UserModel;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::errors::{user_already_exists_error::UserAlreadyExistsError, database_error::DatabaseError};
use password_auth::generate_hash;

pub struct CreateUserParams {
    pub nickname: String,
    pub password: String,
}

pub struct CreateUserService {
    users_repository: SeaUserRepository,
}

#[derive(Debug)]
pub enum CreateUserServiceErrors<UserExist, DB> {
    UserAlreadyExist(UserExist),
    DatabaseConn(DB)
}

impl CreateUserService {
    pub fn new(users_repository: SeaUserRepository) -> Self {
        CreateUserService {
            users_repository
        }
    }

    pub async fn exec(&self, params: CreateUserParams) -> Result<UserModel, CreateUserServiceErrors<UserAlreadyExistsError, DatabaseError>> {
        let user_on_db = &self.users_repository.find_by_nickname(&params.nickname).await;

        if user_on_db.is_err() {
            return Err(CreateUserServiceErrors::DatabaseConn(DatabaseError::new()));
        }

        if let Some(_user) = user_on_db.as_ref().unwrap() {
            return Err(CreateUserServiceErrors::UserAlreadyExist(UserAlreadyExistsError::new(params.nickname)));
        }

        let hashed_password = generate_hash(params.password);

       let created_user = self.users_repository.create(params.nickname, hashed_password).await;

        if user_on_db.is_err() {
            return Err(CreateUserServiceErrors::DatabaseConn(DatabaseError::new()));
        }

        let created_user = created_user.unwrap();

       Ok(created_user)
    }
}