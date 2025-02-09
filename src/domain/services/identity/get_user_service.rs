use uuid::Uuid;

use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub struct GetUserServiceParams {
    pub user_id: Uuid,
}

pub struct GetUserService<UserRepository: UserRepositoryTrait> {
    user_repository: UserRepository,
}

impl<UserRepository: UserRepositoryTrait> GetUserService<UserRepository> {
    pub fn new(user_repository: UserRepository) -> Self {
        GetUserService { user_repository }
    }

    pub async fn exec(&self, params: GetUserServiceParams) -> Result<Option<User>, SamambaiaError> {
        self.user_repository
            .find_by_id(&params.user_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                "Error occurred on Get User Service, while selecting user by Id from the database",
                err,
            )
            })
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;

    #[tokio::test]
    async fn test_get_user_service() {
        let user = User::new("Kaio".into(), "12345".into(), Some(Role::Coord));

        // mocked db
        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
        user_db.lock().unwrap().push(user.clone());

        // mocking user repository
        let mut mocked_user_repository = MockUserRepositoryTrait::new();

        let db_clone = Arc::clone(&user_db);
        mocked_user_repository
            .expect_find_by_id()
            .returning(move |id| {
                let mut found_user: Option<User> = None;

                for user in db_clone.lock().unwrap().clone().into_iter() {
                    if user.id().eq(id) {
                        found_user = Some(user);
                    }
                }

                Ok(found_user)
            });

        // TESTING
        let sut = GetUserService::new(mocked_user_repository);

        let success_result = sut
            .exec(GetUserServiceParams { user_id: user.id() })
            .await
            .unwrap();

        assert!(success_result.is_some());

        let failling_result = sut
            .exec(GetUserServiceParams {
                user_id: Uuid::new_v4(),
            })
            .await
            .unwrap();

        assert!(failling_result.is_none());
    }
}
