use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::{
    core::pagination::PaginationParameters,
    domain::domain_entities::{role::Role, user::User},
};

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindManyUsersResponse(pub Vec<User>, pub u64);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UserQueryType {
    Role(Role),
    Nickname(String),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepositoryTrait {
    // TODO: make it receives a whole User as a parameter just like 'save' method
    async fn create(&self, user: User) -> Result<User, Box<dyn Error>>;

    async fn find_by_nickname(&self, nickname: &str) -> Result<Option<User>, Box<dyn Error>>;

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn Error>>;

    async fn save(&self, user: User) -> Result<User, Box<dyn Error>>;

    async fn find_many(
        &self,
        params: PaginationParameters<UserQueryType>,
    ) -> Result<FindManyUsersResponse, Box<dyn Error>>;
}
