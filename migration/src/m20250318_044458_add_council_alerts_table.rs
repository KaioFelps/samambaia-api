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
                    .table(CouncilAlert::Table)
                    .if_not_exists()
                    .col(pk_auto(CouncilAlert::Id).not_null())
                    .col(string(CouncilAlert::Title).not_null())
                    .col(string(CouncilAlert::Content).not_null())
                    .col(boolean(CouncilAlert::Pinned).default(false).not_null())
                    .col(
                        date_time(CouncilAlert::CreatedAt)
                            .extra("DEFAULT NOW()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CouncilAlert::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CouncilAlert {
    Table,
    Id,
    Title,
    Content,
    Pinned,
    CreatedAt,
}
