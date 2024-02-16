use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .drop_foreign_key(
            ForeignKey::drop()
            .table(Comment::Table)
            .name("fk-article-id")
            .to_owned()
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
        .drop_foreign_key(
            ForeignKey::drop()
            .table(Comment::Table)
            .name("fk-article-id")
            .to_owned()
        )
        .await?;

        manager
        .create_foreign_key(
            ForeignKey::create().name("fk-article-id")
                .from(Comment::Table, Comment::ArticleId)
                .to(Article::Table, Article::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    ArticleId
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id
}