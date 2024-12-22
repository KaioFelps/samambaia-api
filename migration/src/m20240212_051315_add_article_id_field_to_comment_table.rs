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
                    .add_column(ColumnDef::new(Comment::ArticleId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-article-id")
                    .from(Comment::Table, Comment::ArticleId)
                    .to(Article::Table, Article::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Comment::Table)
                    .drop_column(Comment::ArticleId)
                    .to_owned(),
            )
            .await
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
    Id,
}
