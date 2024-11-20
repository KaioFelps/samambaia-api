use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(TeamRoleTeamUser::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
            .await
    }
}

#[derive(DeriveIden)]
enum TeamRoleTeamUser {
    Table,
    Id,
    TeamRoleId,
    TeamUserId,
}

#[derive(DeriveIden)]
enum TeamRole {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TeamUser {
    Table,
    Id,
}
