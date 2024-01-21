use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait HasherTrait {
    fn hash(&self, password: String) -> String;
}