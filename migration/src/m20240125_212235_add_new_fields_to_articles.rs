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
                .drop_column(Article::Likes)
                .add_column_if_not_exists(ColumnDef::new(Article::CoverUrl).string().not_null())
                .add_column_if_not_exists(ColumnDef::new(Article::UpdatedAt).date_time().null())
                .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(Article::Table)
                    .add_column(ColumnDef::new(Article::Likes).integer().default(0))
                    .drop_column(Article::CoverUrl)
                    .drop_column(Article::UpdatedAt)
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    CoverUrl,
    Likes,
    UpdatedAt,
}