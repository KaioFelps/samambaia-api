use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_index(
            Index::create()
            .if_not_exists()
            .col(Article::Slug)
            .table(Article::Table)
            .name("slug_unique_constr_index")
            .unique()
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
            .table(Article::Table)
            .name("slug_unique_constr_index")
            .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Slug
}
