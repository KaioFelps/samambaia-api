use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Announcement::Table)
                    .add_column(string(Announcement::Description))
                    .add_column(date_time(Announcement::CreatedAt))
                    .add_column(date_time_null(Announcement::UpdatedAt))
                    .add_column(uuid(Announcement::AuthorId))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-announcement-author-id")
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction)
                            .from_tbl(Announcement::Table)
                            .from_col(Announcement::AuthorId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Announcement::Table)
                    .drop_foreign_key(Alias::new("fk-announcement-author-id"))
                    .drop_column(Announcement::Description)
                    .drop_column(Announcement::CreatedAt)
                    .drop_column(Announcement::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Announcement {
    Table,
    Description,
    CreatedAt,
    UpdatedAt,
    AuthorId,
}
