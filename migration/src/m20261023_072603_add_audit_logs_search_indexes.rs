use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261023_072603_add_audit_logs_search_indexes"
    }
}

const SEARCH_EXPR: &str = "(coalesce(actor_reference, '') || ' ' || coalesce(target_reference, '') || ' ' || coalesce(target_type, '') || ' ' || action_kind || ' ' || coalesce(actor::text, '') || ' ' || coalesce(target::text, '') || ' ' || coalesce(old_payload::text, '') || ' ' || coalesce(new_payload::text, '') || ' ' || coalesce(metadata::text, ''))";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_trgm")
            .await?;

        db.execute_unprepared("DROP INDEX IF EXISTS idx_audit_logs_search_trgm")
            .await?;

        db.execute_unprepared(&format!(
            "CREATE INDEX idx_audit_logs_search_trgm ON audit_logs \
             USING gin ({SEARCH_EXPR} gin_trgm_ops)"
        ))
        .await?;

        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_audit_logs_happened_at_id \
             ON audit_logs (happened_at DESC, id DESC)",
        )
        .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_audit_logs_happened_at")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_audit_logs_happened_at ON audit_logs (happened_at)",
        )
        .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_audit_logs_happened_at_id")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_audit_logs_search_trgm")
            .await?;

        Ok(())
    }
}
