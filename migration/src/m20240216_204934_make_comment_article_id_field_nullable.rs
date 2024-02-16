use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .alter_table(
            Table::alter()
                .table(Comment::Table)
                .drop_column(Comment::ArticleId)
                .to_owned()
        ).await?;

        manager
        .alter_table(
            Table::alter()
                .table(Comment::Table)
                .add_column(ColumnDef::new(Comment::ArticleId).uuid().null())
                .to_owned(),
        )
        .await?;

        manager
            .create_foreign_key(
                ForeignKey::create().name("fk-article-id")
                    .from(Comment::Table, Comment::ArticleId)
                    .to(Article::Table, Article::Id)
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .alter_table(
            Table::alter()
                .table(Comment::Table)
                .drop_column(Comment::ArticleId)
                .to_owned()
        ).await?;
        
        manager
        .alter_table(
            Table::alter()
                .table(Comment::Table)
                .add_column(ColumnDef::new(Comment::ArticleId).uuid().not_null().default("98afa6c5-71c3-4d44-a731-a54c6adf0c6e"))
                .to_owned(),
        )
        .await?;

        manager
        .create_foreign_key(
            ForeignKey::create().name("fk-article-id")
                .from(Comment::Table, Comment::ArticleId)
                .to(Article::Table, Article::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    ArticleId,
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id
}