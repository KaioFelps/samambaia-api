use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Article::Table)
                    .add_column_if_not_exists(ColumnDef::new(Article::Approved).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(Article::Table)
                .drop_column(Article::Approved)
                .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Approved
}