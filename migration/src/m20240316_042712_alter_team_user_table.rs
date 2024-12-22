use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TeamUser::Table)
                    .add_column(ColumnDef::new(TeamUser::TeamRoleId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamUser::Table)
                    .rename_column(TeamUser::Task, TeamUser::UserFunction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-team-role-id")
                    .from(TeamUser::Table, TeamUser::TeamRoleId)
                    .to(TeamRole::Table, TeamRole::Id)
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
                    .table(TeamUser::Table)
                    .rename_column(TeamUser::UserFunction, TeamUser::Task)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamUser::Table)
                    .drop_foreign_key(Alias::new("fk-team-role-id"))
                    .drop_column(TeamUser::TeamRoleId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TeamRole {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TeamUser {
    Table,
    TeamRoleId,
    Task,
    UserFunction,
}
