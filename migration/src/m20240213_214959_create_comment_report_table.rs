use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommentReport::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommentReport::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CommentReport::CommentId).uuid().not_null())
                    .col(ColumnDef::new(CommentReport::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(CommentReport::Message)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(CommentReport::Solved)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(CommentReport::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-id")
                            .from(CommentReport::Table, CommentReport::CommentId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-id")
                            .from(CommentReport::Table, CommentReport::CommentId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommentReport::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommentReport {
    Table,
    Id,
    CommentId,
    UserId,
    Message,
    Solved,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
