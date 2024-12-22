use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FreeBadge::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FreeBadge::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FreeBadge::Image).string().not_null())
                    .col(ColumnDef::new(FreeBadge::Code).string().not_null())
                    .col(ColumnDef::new(FreeBadge::Link).string().not_null())
                    .col(
                        ColumnDef::new(FreeBadge::LinkIsExternal)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(FreeBadge::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(ColumnDef::new(FreeBadge::AvailableUntil).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FreeBadge::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FreeBadge {
    Table,
    Id,
    Image,
    Code,
    Link,
    LinkIsExternal,
    CreatedAt,
    AvailableUntil,
}
