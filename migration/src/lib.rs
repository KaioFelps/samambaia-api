pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240114_032712_add_roles_to_user;
mod m20240125_212235_add_new_fields_to_articles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240114_032712_add_roles_to_user::Migration),
            Box::new(m20240125_212235_add_new_fields_to_articles::Migration),
        ]
    }
}
