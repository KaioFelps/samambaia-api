use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261023_072026_add_audit_logs_columns"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogs::Table)
                    .add_column_if_not_exists(string_null(AuditLogs::TargetType))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db.execute_unprepared("ALTER TABLE audit_logs ALTER COLUMN actor DROP NOT NULL")
            .await?;

        // this extract target type from the target_reference of audit logs rows that has
        // target_reference like "some title (Article)", etc
        db.execute_unprepared(
            r#"
            UPDATE audit_logs
            SET target_type = substring(target_reference from '\(([^()]*)\)$')
            WHERE target_type IS NULL AND target_reference IS NOT NULL
            "#,
        )
        .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_audit_logs_target_type")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::TargetType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_audit_logs_target_type")
                    .table(AuditLogs::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogs::Table)
                    .drop_column(AuditLogs::TargetType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum AuditLogs {
    Table,
    TargetType,
}
