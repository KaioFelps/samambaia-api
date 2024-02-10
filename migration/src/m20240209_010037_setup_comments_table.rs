use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // COMMENT TABLE
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comment::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Comment::AuthorId).uuid().not_null())
                    .foreign_key(ForeignKey::create().name("fk-comment-author-id")
                        .from(Comment::Table, Comment::AuthorId)
                        .to(User::Table, User::Id))
                    .col(ColumnDef::new(Comment::Content).text().not_null().default(""))
                    .col(ColumnDef::new(Comment::CreatedAt).date_time().not_null().extra("DEFAULT NOW()"))
                    .to_owned(),
            )
            .await?;

            // COMMENT_ARTICLE TABLE
            manager
            .create_table(
                Table::create()
                    .table(CommentArticle::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CommentArticle::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CommentArticle::ArticleId).uuid().not_null())
                    .col(ColumnDef::new(CommentArticle::CommentId).uuid().not_null())
                    .foreign_key(ForeignKey::create().name("fk-comment-id")
                        .from(CommentArticle::Table, CommentArticle::CommentId)
                        .to(Comment::Table, Comment::Id)
                        .on_delete(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk-article-id")
                        .from(CommentArticle::Table, CommentArticle::ArticleId)
                        .to(Article::Table, Article::Id)
                        .on_delete(ForeignKeyAction::Cascade))
                    .to_owned()
            ).await?;

            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(CommentArticle::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    AuthorId,
    Content,
    CreatedAt,
}

#[derive(DeriveIden)]
enum CommentArticle {
    Table,
    Id,
    ArticleId,
    CommentId
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id
}