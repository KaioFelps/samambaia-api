use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .create_table(
            Table::create()
            .table(User::Table)
            .if_not_exists()
            .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(User::Nickname).string().unique_key().not_null())
            .col(ColumnDef::new(User::Password).string().not_null())
            .col(ColumnDef::new(User::CreatedAt).date_time().not_null().extra("DEFAULT NOW()"))
            .col(ColumnDef::new(User::LastLogin).date_time().null())
            .to_owned()
        ).await?;
        
        manager
            .create_table(
                Table::create()
                .table(Article::Table)
                .if_not_exists()
                .col(ColumnDef::new(Article::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Article::Title).string().not_null())
                .col(ColumnDef::new(Article::Content).text().default(""))
                .col(ColumnDef::new(Article::AuthorId).uuid().not_null())
                .foreign_key(ForeignKey::create().name("fk-article-author-id")
                    .from(Article::Table, Article::AuthorId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade))
                .col(ColumnDef::new(Article::Likes).integer().default(0))
                .col(ColumnDef::new(Article::CreatedAt).date_time().not_null().extra("DEFAULT NOW()"))
                .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    Title,
    Content,
    AuthorId,
    Likes,
    CreatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Nickname,
    Password,
    CreatedAt,
    LastLogin,
}