use std::error::Error;

use log::error;
use uuid::Uuid;

use crate::domain::cryptography::hasher::HasherTrait;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::util::verify_role_hierarchy_matches;
use crate::util::verify_role_has_permission;

use crate::{LOG_SEP, R_EOL};

pub struct UpdateUserParams {
    pub staff_id: Uuid,
    pub staff_role: Role,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>
}
pub struct UpdateUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher: Box<dyn HasherTrait>
}

impl<UserRepositoryType: UserRepositoryTrait> UpdateUserService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, hasher: Box<dyn HasherTrait>) -> Self {
        UpdateUserService {
            user_repository,
            hasher
        }
    }

    pub async fn exec(&self, params: UpdateUserParams) -> Result<User, Box<dyn Error>> {
        let staff_can_update_user = verify_role_has_permission(&params.staff_role, crate::util::RolePermissions::UpdateUser);
        
        if !staff_can_update_user {
            return Err(Box::new(UnauthorizedError::new()));
        }
        
        let user = self.user_repository.find_by_id(&params.user_id).await;
        
        if user.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Update User Service, while finding the user by id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }
        
        if let None = user.as_ref().unwrap() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }
        
        let mut user = user.unwrap().unwrap();

        let operation_follows_role_hierarchy = verify_role_hierarchy_matches(
            &user.role().as_ref().unwrap(),
            &params.staff_role
        );

        if !operation_follows_role_hierarchy {
            return Err(
                Box::new(UnauthorizedError::new())
            );
        }

        user.set_nickname(if params.nickname.is_some() { params.nickname.unwrap() } else { user.nickname().to_string() });

        user.set_password(if params.password.is_some() {
            let hashed_password = self.hasher.hash(params.password.unwrap());
            hashed_password
        } else {user.password().to_string()});

        user.set_role(if params.role.is_some() { params.role } else { user.role() });

        let result = self.user_repository.save(user).await;

        match result {
            Ok(_) => (),
            Err(err) => {
                error!(
                    "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Update User Service, while saving the user on the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                    err
                );

                return Err(Box::new(InternalError::new()));
            }
        };

        let user = result.unwrap();

       Ok(user)
    }
}