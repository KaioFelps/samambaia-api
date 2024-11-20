pub fn get_repository_flat_template(capitalized_name: &String) -> String {
    format!(
        r#"use async_trait::async_trait;
use std::error::Error;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait {capitalized_name}RepositoryTrait {{}}
"#
    )
}
