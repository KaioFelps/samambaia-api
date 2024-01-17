use entities::user::Column;
use entities::user::ActiveModel;
use entities::user::Model as UserModel;
use sea_orm::ActiveModelTrait;
use sea_orm::TryIntoModel;
use uuid::Uuid;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use entities::sea_orm_active_enums::Role as UserRole;
use password_auth::generate_hash;
use crate::util::verify_role_hierarchy_matches;

pub struct UpdateUserParams {
    pub staff_id: Uuid,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>
}
pub struct UpdateUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
}

#[derive(Debug)]
pub enum UpdateUserServiceErrors<Internal, UnAuth, NFound> {
    InternalError(Internal),
    Unauthorized(UnAuth),
    NotFound(NFound)
}

impl<UserRepositoryType: UserRepositoryTrait> UpdateUserService<UserRepositoryType> {
    pub fn new(user_repository: Box<UserRepositoryType>) -> Self {
        UpdateUserService {
            user_repository
        }
    }

    pub async fn exec(&self, params: UpdateUserParams) -> Result<UserModel, UpdateUserServiceErrors<InternalError, UnauthorizedError, ResourceNotFoundError>> {
        let staff_on_db = self.user_repository.find_by_id(&params.staff_id).await;

        if staff_on_db.is_err() {
            return Err(UpdateUserServiceErrors::InternalError(InternalError::new()));
        }

        let staff_on_db = staff_on_db.unwrap();

        if staff_on_db.is_none() {
            return Err(UpdateUserServiceErrors::Unauthorized(UnauthorizedError::new()));
        }

        let staff_on_db = staff_on_db.unwrap();

        match staff_on_db.role {
            Some(UserRole::Admin) => (),
            Some(UserRole::Ceo) => (),
            Some(UserRole::Principal) => (),
            _ => return Err(UpdateUserServiceErrors::Unauthorized(UnauthorizedError::new()))
        }

        
        let user = self.user_repository.find_by_id(&params.user_id).await;
        
        
        if user.is_err() {
            return Err(UpdateUserServiceErrors::InternalError(InternalError::new()));
        }
        
        if let None = user.as_ref().unwrap() {
            return Err(UpdateUserServiceErrors::NotFound(ResourceNotFoundError::new()));
        }
        
        let user = user.unwrap().unwrap();

        let operation_follows_role_hierarchy = verify_role_hierarchy_matches(
            &user.role.as_ref().unwrap(),
            &staff_on_db.role.as_ref().unwrap()
        );

        if !operation_follows_role_hierarchy {
            return Err(
                UpdateUserServiceErrors::Unauthorized(UnauthorizedError::new())
            );
        }

        let mut user = ActiveModel::from(user.to_owned());

        user.set(Column::Nickname, if params.nickname.is_some() {params.nickname.unwrap().to_owned().into()} else {user.nickname.clone().unwrap().into()});

        user.set(Column::Password, if params.password.is_some() {
            let hashed_password = generate_hash(params.password.unwrap());

            hashed_password.to_owned().into()
        } else {user.password.clone().unwrap().into()});

        user.set(Column::Role, if params.role.is_some() {params.role.unwrap().to_owned().into()} else {user.role.clone().unwrap().into()});

        let result = self.user_repository.save(&user).await;

        match result {
            Ok(_) => (),
            Err(_err) => {
                return Err(UpdateUserServiceErrors::InternalError(InternalError::new()));
            }
        }

        let user = user.try_into_model().unwrap();

       Ok(user)
    }
}