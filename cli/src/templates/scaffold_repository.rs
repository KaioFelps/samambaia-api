pub fn get_repository_flat_template(
    capitalized_entity: &String,
    entity_file_name: &String,
) -> String {
    return format!(
        r#"use async_trait::async_trait;
use std::error::Error;

use crate::domain::domain_entities::{entity_file_name}::{capitalized_entity};
use crate::core::pagination::PaginationParameters;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct FindMany{capitalized_entity}sResponse (
    pub Vec<{capitalized_entity}>,
    pub u64,
);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum {capitalized_entity}QueryType {{
    // CONTENT(String),
}}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait {capitalized_entity}RepositoryTrait {{
    async fn create(&self, {entity_file_name}: {capitalized_entity}) -> Result<{capitalized_entity}, Box<dyn Error>>;

    async fn find_by_id(&self, comm_report_id: i32) -> Result<Option<{capitalized_entity}>, Box<dyn Error>>;

    async fn find_many(&self, params: PaginationParameters<{capitalized_entity}QueryType>) -> Result<FindMany{capitalized_entity}sResponse, Box<dyn Error>>;

    async fn save(&self, {entity_file_name}: {capitalized_entity}) -> Result<{capitalized_entity}, Box<dyn Error>>;

    async fn delete(&self, {entity_file_name}: {capitalized_entity}) -> Result<(), Box<dyn Error>>;
}}
"#
    );
}
