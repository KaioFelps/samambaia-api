use uuid::Uuid;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::domain::cryptography::both::HasherAndComparerTrait;

pub struct ChangePasswordParams {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}
pub struct ChangePasswordService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher_and_comparer: Box<dyn HasherAndComparerTrait>
}

#[derive(Debug)]
pub enum ChangePasswordServiceErrors<Internal, UnAuth, NFound> {
    InternalError(Internal),
    Unauthorized(UnAuth),
    NotFound(NFound)
}

impl<UserRepositoryType: UserRepositoryTrait> ChangePasswordService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>, hasher_and_comparer: Box<dyn HasherAndComparerTrait>) -> Self {
        ChangePasswordService {
            user_repository,
            hasher_and_comparer
        }
    }

    pub async fn exec(&self, params: ChangePasswordParams) -> Result<(), ChangePasswordServiceErrors<InternalError, UnauthorizedError, ResourceNotFoundError>> {
        let user_on_db = self.user_repository.find_by_id(&params.user_id).await;
                
        if user_on_db.is_err() {
            return Err(ChangePasswordServiceErrors::InternalError(InternalError::new()));
        }
        
        if let None = user_on_db.as_ref().unwrap() {
            return Err(ChangePasswordServiceErrors::NotFound(ResourceNotFoundError::new()));
        }
        
        let mut user = user_on_db.unwrap().unwrap();

        let password_matches = self.hasher_and_comparer.compare(&params.current_password, &user.password().to_string());

        if !password_matches {
            return Err(ChangePasswordServiceErrors::Unauthorized(UnauthorizedError::new()));
        }

        let new_password = self.hasher_and_comparer.hash(params.new_password);

        user.set_password(new_password);

        let result = self.user_repository.save(user).await;

        match result {
            Ok(_) => return Ok(()),
            Err(_err) => {
                return Err(ChangePasswordServiceErrors::InternalError(InternalError::new()));
            }
        }
    }
}