use std::error::Error;
use std::future::Future;
use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;

pub trait UserRepositoryTrait {
    fn create(&self, nickname: String, password: String, role: Role) -> impl Future<Output = Result<User, Box<dyn Error>>>;

    fn find_by_nickname(&self, nickname: &String) -> impl Future<Output = Result<Option<User>, Box<dyn Error>>>;

    fn find_by_id(&self, id: &Uuid) -> impl Future<Output = Result<Option<User>, Box<dyn Error>>>;

    fn save(&self, user: User) -> impl Future<Output = Result<User, Box<dyn Error>>>;
}