use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ArticleTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ArticleTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ArticleTag::Value)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Article::Table)
                    .add_column(ColumnDef::new(Article::TagId).integer())
                    .add_column(ColumnDef::new(Article::TagValue).string())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-article-article-tag")
                            .from_tbl(Article::Table)
                            .from_col(Article::TagId)
                            .to_tbl(ArticleTag::Table)
                            .to_col(ArticleTag::Id)
                            .on_delete(ForeignKeyAction::SetNull.to_owned()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .table(Article::Table)
                    .name("fk-article-article-tag")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ArticleTag::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Article::Table)
                    .drop_column(Article::TagValue)
                    .drop_column(Article::TagId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ArticleTag {
    Table,
    Id,
    Value,
}

#[derive(DeriveIden)]
enum Article {
    Table,
    TagId,
    TagValue,
}
