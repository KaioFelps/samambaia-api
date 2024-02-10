use std::error::Error;
use log::error;
use uuid::Uuid;

use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::domain::cryptography::both::HasherAndComparerTrait;

use crate::{LOG_SEP, R_EOL};

pub struct ChangePasswordParams {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}
pub struct ChangePasswordService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher_and_comparer: Box<dyn HasherAndComparerTrait>
}

impl<UserRepositoryType: UserRepositoryTrait> ChangePasswordService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, hasher_and_comparer: Box<dyn HasherAndComparerTrait>) -> Self {
        ChangePasswordService {
            user_repository,
            hasher_and_comparer
        }
    }

    pub async fn exec(&self, params: ChangePasswordParams) -> Result<(), Box<dyn Error>> {
        let user_on_db = self.user_repository.find_by_id(&params.user_id).await;
                
        if user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Change Password Service, while fetching user from database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user_on_db.as_ref().unwrap_err()
            );
            
            return Err(Box::new(InternalError::new()));
        }
        
        if let None = user_on_db.as_ref().unwrap() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }
        
        let mut user = user_on_db.unwrap().unwrap();

        let password_matches = self.hasher_and_comparer.compare(&params.current_password, &user.password().to_string());

        if !password_matches {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let new_password = self.hasher_and_comparer.hash(params.new_password);

        user.set_password(new_password);

        let result = self.user_repository.save(user).await;

        match result {
            Ok(_) => return Ok(()),
            Err(_err) => {
                return Err(Box::new(InternalError::new()));
            }
        }
    }
}