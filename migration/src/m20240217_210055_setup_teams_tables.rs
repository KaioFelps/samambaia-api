use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TeamRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TeamRole::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(TeamRole::Title).string().not_null())
                    .col(ColumnDef::new(TeamRole::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TeamUser::Id).uuid().primary_key().not_null())
                    .col(
                        ColumnDef::new(TeamUser::Nickname)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TeamUser::Task).string().not_null())
                    .col(ColumnDef::new(TeamUser::Twitter).string().null())
                    .col(ColumnDef::new(TeamUser::Discord).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamRoleTeamUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TeamRoleTeamUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TeamRoleTeamUser::TeamRoleId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TeamRoleTeamUser::TeamUserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team-role-id")
                            .from(TeamRoleTeamUser::Table, TeamRoleTeamUser::TeamRoleId)
                            .to(TeamRole::Table, TeamRole::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team-user-id")
                            .from(TeamRoleTeamUser::Table, TeamRoleTeamUser::TeamUserId)
                            .to(TeamUser::Table, TeamUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-team-user-id")
                    .table(TeamRoleTeamUser::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-team-role-id")
                    .table(TeamRoleTeamUser::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TeamRoleTeamUser::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TeamRole::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TeamUser::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TeamRole {
    Table,
    Id,
    Title,
    Description,
}

#[derive(DeriveIden)]
enum TeamUser {
    Table,
    Id,
    Nickname,
    Task,
    Twitter,
    Discord,
}

#[derive(DeriveIden)]
enum TeamRoleTeamUser {
    Table,
    Id,
    TeamRoleId,
    TeamUserId,
}
