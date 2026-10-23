use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261023_051522_add_audit_logs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditLogs::Table)
                    .if_not_exists()
                    .col(big_pk_auto(AuditLogs::Id))
                    .col(string_null(AuditLogs::ActorReference))
                    .col(string_null(AuditLogs::TargetReference))
                    .col(string(AuditLogs::ActionKind))
                    .col(timestamp_with_time_zone(AuditLogs::HappenedAt))
                    .col(
                        timestamp_with_time_zone(AuditLogs::LoggedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(json_binary(AuditLogs::Actor))
                    .col(json_binary_null(AuditLogs::Target))
                    .col(json_binary_null(AuditLogs::OldPayload))
                    .col(json_binary_null(AuditLogs::NewPayload))
                    .col(json_binary_null(AuditLogs::Metadata))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_happened_at")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::HappenedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_actor_reference")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::ActorReference)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_target_reference")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::TargetReference)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_action_kind")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::ActionKind)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AuditLogs {
    Table,
    Id,
    ActorReference,
    TargetReference,
    ActionKind,
    HappenedAt,
    LoggedAt,
    Actor,
    Target,
    OldPayload,
    NewPayload,
    Metadata,
}
