use entities::user::Column;
use entities::user::ActiveModel;
use password_auth::verify_password;
use sea_orm::ActiveModelTrait;
use sea_orm::Value;
use uuid::Uuid;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::repositories::user_repository::UserRepositoryTrait;
use password_auth::generate_hash;

pub struct ChangePasswordParams {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}
pub struct ChangePasswordService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
}

#[derive(Debug)]
pub enum ChangePasswordServiceErrors<Internal, UnAuth, NFound> {
    InternalError(Internal),
    Unauthorized(UnAuth),
    NotFound(NFound)
}

impl<UserRepositoryType: UserRepositoryTrait> ChangePasswordService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>) -> Self {
        ChangePasswordService {
            user_repository
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
        
        let user = user_on_db.unwrap().unwrap();

        let password_matches = verify_password(params.current_password, &user.password);

        match password_matches {
            Ok(_) => (),
            Err(_err) => return Err(ChangePasswordServiceErrors::Unauthorized(UnauthorizedError::new())),
        }

        let mut user = ActiveModel::from(user.to_owned());

        let new_password = generate_hash(params.new_password);

        user.set(Column::Password, Value::from(new_password));

        let result = self.user_repository.save(&user).await;

        match result {
            Ok(_) => return Ok(()),
            Err(_err) => {
                return Err(ChangePasswordServiceErrors::InternalError(InternalError::new()));
            }
        }
    }
}