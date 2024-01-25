use std::error::Error;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::domain_entities::user::User;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepositoryTrait {
    // TODO: make it receives a whole User as a parameter just like 'save' method
    async fn create(&self, user: User) -> Result<User, Box<dyn Error>>;
    
    async fn find_by_nickname(&self, nickname: &String) -> Result<Option<User>, Box<dyn Error>>;
    
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn Error>>;
    
    async fn save(&self, user: User) -> Result<User, Box<dyn Error>>;
}

#[cfg(test)]
mod test {
    use tokio;
    use crate::domain::domain_entities::role::Role;
    use super::*;

    #[tokio::test]
    async fn create() {
        let mut db: Vec<User> = vec![];
        let mut mocked_repo = MockUserRepositoryTrait::default();

        let nickname = "Floricultor".to_string();
        let password = "123456".to_string();
        let role = Some(Role::Ceo);

        mocked_repo
        .expect_create()
        .returning(move |user: User| {
            db.push(user);

            Ok(db[0].clone())
        });

        let user = User::new(nickname, password, role);
        let result = mocked_repo.create(user).await;

        assert!(!result.unwrap().id().is_nil());
    }

    #[tokio::test]
    async fn find_by_nickname() {
        let mut db: Vec<User> = vec![];
        let mut mocked_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::default();

        let nickname = "Floricultor".to_string();
        let password = "123456".to_string();
        let role = Some(Role::Ceo);

        let user = User::new(nickname.clone(), password, role);

        db.push(user.clone());

        mocked_repo
        .expect_find_by_nickname()
        .returning(move |param_nickname| {
            let mut index: Option<u8> = None;
            
            for (i, item) in db.clone().into_iter().enumerate() {
                if item.nickname() == param_nickname {
                    index = Some(i as u8);
                    break;
                }
            }

            if index.is_none() {
                return Ok(None);
            }

            let user = db[index.unwrap() as usize].clone();

            Ok(Some(user))
        });

        let result = mocked_repo.find_by_nickname(&nickname).await;

        assert_eq!(user, result.unwrap().unwrap());
    }

    #[tokio::test]
    async fn find_by_id() {
        let mut db: Vec<User> = vec![];
        let mut mocked_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::default();

        let nickname = "Floricultor".to_string();
        let password = "123456".to_string();
        let role = Some(Role::Ceo);

        let user = User::new(nickname.clone(), password, role);

        db.push(user.clone());

        mocked_repo
        .expect_find_by_id()
        .returning(move |param_id| {
            let mut index: Option<usize> =  None;
            
            for (i, item) in db.clone().into_iter().enumerate() {
                if item.id().to_string() == param_id.to_string() {
                    index = Some(i);
                    break;
                }
            };

            if index.is_none() {
                return Ok(None)
            }

            let db_user = db[index.unwrap()].clone();

            Ok(Some(db_user))
        });

        let result = mocked_repo.find_by_id(&user.id()).await;

        assert_eq!(user.id(), result.unwrap().unwrap().id());

        let result = mocked_repo.find_by_id(&Uuid::new_v4()).await;

        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn save() {
        let mut db: Vec<User> = vec![];
        let mut mocked_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::default();

        let nickname = "Floricultor".to_string();
        let password = "123456".to_string();
        let role = Some(Role::Ceo);

        let user = User::new(nickname, password, role);

        let mut new_user = user.clone();

        new_user.set_nickname("Floricultora".to_string());

        mocked_repo
        .expect_save()
        .returning(move |new_user| {
            db.push(new_user.clone());
            Ok(new_user.to_owned())
        })
        .times(1);

        let result = mocked_repo.save(new_user).await;

        assert_ne!(user.nickname(), result.unwrap().nickname());
    }
}