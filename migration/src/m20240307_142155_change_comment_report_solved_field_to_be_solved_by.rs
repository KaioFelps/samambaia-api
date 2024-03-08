use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(CommentReport::Table)
                .drop_column(CommentReport::Solved)
                .add_column(ColumnDef::new(CommentReport::SolvedBy)
                    .null().uuid()
                )
                .to_owned()
            )
            .await?;

            manager
            .create_foreign_key(
                ForeignKey::create()
                .name("fk-solved-by-user-id")
                .from(CommentReport::Table, CommentReport::SolvedBy)
                .to(User::Table, User::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .to_owned()
            ).await?;

            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                .table(CommentReport::SolvedBy)
                .name("fk-solved-by-user-id")
                .to_owned()
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                .table(CommentReport::Table)
                .drop_column(CommentReport::SolvedBy)
                .add_column(ColumnDef::new(CommentReport::Solved)
                    .boolean().not_null().default(false)
                )
                .to_owned()
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum CommentReport {
    Table,
    SolvedBy,
    Solved
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id
}