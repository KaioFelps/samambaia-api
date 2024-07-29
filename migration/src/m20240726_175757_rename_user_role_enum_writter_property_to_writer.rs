use sea_orm_migration::prelude::*;
use crate::extension::postgres::Type;
use crate::sea_orm::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_type(
                Type::alter()
                    .name(Role::Table)
                    .rename_value(Alias::new("Writter"), Alias::new("Writer"))
                    .to_owned()
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_type(
            Type::alter()
                .name(Role::Table)
                .rename_value(Alias::new("Writer"), Alias::new("Writter"))
                .to_owned()
        ).await
    }
}

#[derive(Iden, EnumIter)]
pub enum Role {
    Table
}