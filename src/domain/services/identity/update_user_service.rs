use crate::domain::cryptography::hasher::HasherTrait;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::DomainError;
use crate::util::generate_service_internal_error;
use crate::util::verify_role_has_permission;
use crate::util::verify_role_hierarchy_matches;
use uuid::Uuid;

pub struct UpdateUserParams {
    pub staff_id: Uuid,
    pub staff_role: Role,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
}
pub struct UpdateUserService<UserRepository: UserRepositoryTrait, Hasher: HasherTrait> {
    user_repository: UserRepository,
    hasher: Hasher,
}

impl<UserRepositoryType: UserRepositoryTrait, Hasher: HasherTrait>
    UpdateUserService<UserRepositoryType, Hasher>
{
    pub fn new(user_repository: UserRepositoryType, hasher: Hasher) -> Self {
        UpdateUserService {
            user_repository,
            hasher,
        }
    }

    pub async fn exec(&self, params: UpdateUserParams) -> Result<User, DomainError> {
        let staff_can_update_user = verify_role_has_permission(
            &params.staff_role,
            crate::util::RolePermissions::UpdateUser,
        );

        if !staff_can_update_user {
            return Err(DomainError::unauthorized_err());
        }

        let mut user = match self
            .user_repository
            .find_by_id(&params.user_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Update User Service, while finding the user by id",
                    err,
                )
            })? {
            None => return Err(DomainError::resource_not_found_err()),
            Some(user) => user,
        };

        let operation_follows_role_hierarchy =
            verify_role_hierarchy_matches(user.role().as_ref().unwrap(), &params.staff_role);

        if !operation_follows_role_hierarchy {
            return Err(DomainError::unauthorized_err());
        }

        user.set_nickname(if params.nickname.is_some() {
            params.nickname.unwrap()
        } else {
            user.nickname().to_string()
        });

        user.set_password(if params.password.is_some() {
            self.hasher.hash(params.password.unwrap())
        } else {
            user.password().to_string()
        });

        user.set_role(if params.role.is_some() {
            params.role
        } else {
            user.role()
        });

        self.user_repository.save(user).await.map_err(|err| {
            generate_service_internal_error(
                "Error occurred on Update User Service, while saving the user on the database",
                err,
            )
        })
    }
}
