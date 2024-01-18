use uuid::Uuid;
use crate::domain::cryptography::hasher::HasherTrait;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::util::verify_role_hierarchy_matches;

pub struct UpdateUserParams {
    pub staff_id: Uuid,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>
}
pub struct UpdateUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher: Box<dyn HasherTrait>
}

#[derive(Debug)]
pub enum UpdateUserServiceErrors<Internal, UnAuth, NFound> {
    InternalError(Internal),
    Unauthorized(UnAuth),
    NotFound(NFound)
}

impl<UserRepositoryType: UserRepositoryTrait> UpdateUserService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, hasher: Box<dyn HasherTrait>) -> Self {
        UpdateUserService {
            user_repository,
            hasher
        }
    }

    pub async fn exec(&self, params: UpdateUserParams) -> Result<User, UpdateUserServiceErrors<InternalError, UnauthorizedError, ResourceNotFoundError>> {
        let staff_on_db = self.user_repository.find_by_id(&params.staff_id).await;

        if staff_on_db.is_err() {
            return Err(UpdateUserServiceErrors::InternalError(InternalError::new()));
        }

        let staff_on_db = staff_on_db.unwrap();

        if staff_on_db.is_none() {
            return Err(UpdateUserServiceErrors::Unauthorized(UnauthorizedError::new()));
        }

        let staff_on_db = staff_on_db.unwrap();

        match staff_on_db.role() {
            Some(Role::Admin) => (),
            Some(Role::Ceo) => (),
            Some(Role::Principal) => (),
            _ => return Err(UpdateUserServiceErrors::Unauthorized(UnauthorizedError::new()))
        }
        
        let user = self.user_repository.find_by_id(&params.user_id).await;
        
        if user.is_err() {
            return Err(UpdateUserServiceErrors::InternalError(InternalError::new()));
        }
        
        if let None = user.as_ref().unwrap() {
            return Err(UpdateUserServiceErrors::NotFound(ResourceNotFoundError::new()));
        }
        
        let mut user = user.unwrap().unwrap();

        let operation_follows_role_hierarchy = verify_role_hierarchy_matches(
            &user.role().as_ref().unwrap(),
            &staff_on_db.role().as_ref().unwrap()
        );

        if !operation_follows_role_hierarchy {
            return Err(
                UpdateUserServiceErrors::Unauthorized(UnauthorizedError::new())
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
            Err(_err) => {
                return Err(UpdateUserServiceErrors::InternalError(InternalError::new()));
            }
        };

        let user = result.unwrap();

       Ok(user)
    }
}