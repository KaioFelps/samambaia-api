use log::error;
use uuid::Uuid;

use crate::domain::cryptography::both::HasherAndComparerTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::invalid_credentials_error::InvalidCredentialsError;
use crate::errors::resource_not_found::ResourceNotFoundError;

use crate::{LOG_SEP, R_EOL};

pub struct ChangePasswordParams {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}
pub struct ChangePasswordService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>,
    hasher_and_comparer: Box<dyn HasherAndComparerTrait>,
}

impl<UserRepositoryType: UserRepositoryTrait> ChangePasswordService<UserRepositoryType> {
    pub fn new(
        user_repository: Box<UserRepositoryType>,
        hasher_and_comparer: Box<dyn HasherAndComparerTrait>,
    ) -> Self {
        ChangePasswordService {
            user_repository,
            hasher_and_comparer,
        }
    }

    pub async fn exec(
        &self,
        params: ChangePasswordParams,
    ) -> Result<(), Box<dyn DomainErrorTrait>> {
        let user_on_db = self.user_repository.find_by_id(&params.user_id).await;

        if user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Change Password Service, while fetching user from database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        if user_on_db.as_ref().unwrap().is_none() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let mut user = user_on_db.unwrap().unwrap();

        let password_matches = self
            .hasher_and_comparer
            .compare(&params.current_password, user.password());

        if !password_matches {
            return Err(Box::new(InvalidCredentialsError::new()));
        }

        let new_password = self.hasher_and_comparer.hash(params.new_password);

        user.set_password(new_password);

        let result = self.user_repository.save(user).await;

        match result {
            Ok(_) => Ok(()),
            Err(_err) => Err(Box::new(InternalError::new())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{
        cryptography::{comparer::ComparerTrait, hasher::HasherTrait},
        domain_entities::{role::Role, user::User},
        repositories::user_repository::MockUserRepositoryTrait,
    };
    use crate::infra::cryptography::MockedAuthHasherAndVerifier;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        let fake_hasher = MockedAuthHasherAndVerifier;

        // instantiating needed entities
        let hashed_initial_password = fake_hasher.hash("123456".into());
        let user = User::new(
            "Floricultor".into(),
            hashed_initial_password,
            Some(Role::User),
        );

        // mocking the database
        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
        user_db.lock().unwrap().push(user.clone());

        // mocking the repository
        let mut mocked_user_repository = MockUserRepositoryTrait::new();

        let db_clone = Arc::clone(&user_db);
        mocked_user_repository
            .expect_find_by_id()
            .returning(move |id| {
                for user in db_clone.lock().unwrap().iter() {
                    if user.id().eq(id) {
                        return Ok(Some(user.clone()));
                    }
                }

                Ok(None)
            });

        let db_clone = Arc::clone(&user_db);
        mocked_user_repository
            .expect_save()
            .returning(move |param_user| {
                let mut index = None;

                for (i, tu) in db_clone.lock().unwrap().iter().enumerate() {
                    if tu.id().eq(&param_user.id()) {
                        index = Some(i);
                    }
                }

                db_clone.lock().unwrap()[index.unwrap()] = param_user.clone();
                Ok(param_user)
            });

        // testing
        let sut = ChangePasswordService::new(
            Box::new(mocked_user_repository),
            Box::new(fake_hasher.clone()),
        );

        let res = sut
            .exec(ChangePasswordParams {
                current_password: "123456".into(),
                new_password: "1234a".into(),
                user_id: user.id(),
            })
            .await;

        assert!(res.is_ok());

        assert!(fake_hasher.compare("1234a", user_db.lock().unwrap()[0].password()));
    }
}
