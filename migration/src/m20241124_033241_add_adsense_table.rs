use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Announcement::Table)
                    .if_not_exists()
                    .col(pk_uuid(Announcement::Id).not_null())
                    .col(string(Announcement::Url))
                    .col(string(Announcement::Image))
                    .col(boolean(Announcement::External))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Announcement::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Announcement {
    Table,
    Id,
    Url,
    Image,
    External,
}
