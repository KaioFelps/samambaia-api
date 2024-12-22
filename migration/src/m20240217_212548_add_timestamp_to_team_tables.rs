use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TeamRole::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(TeamRole::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamUser::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(TeamUser::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamRoleTeamUser::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(TeamRoleTeamUser::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TeamRole::Table)
                    .drop_column(TeamRole::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamUser::Table)
                    .drop_column(TeamUser::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamRoleTeamUser::Table)
                    .drop_column(TeamRoleTeamUser::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TeamRole {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TeamUser {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TeamRoleTeamUser {
    Table,
    CreatedAt,
}
