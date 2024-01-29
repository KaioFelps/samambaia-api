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
