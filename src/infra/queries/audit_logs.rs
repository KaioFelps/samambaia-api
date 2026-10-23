use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Postgres, QueryBuilder};

use crate::core::pagination::{MAX_PER_PAGE, MAX_SEARCH_LEN, PaginationResponse};
use crate::domain::domain_entities::role::Role;
use crate::error::SamambaiaError;
use crate::infra::http::presenters::audit_log::{AuditLogPresenter, PresentedLog};
use crate::infra::http::presenters::presenter::{JsonWrappedPaginatedEntity, PresenterTrait};
use crate::infra::sea::sea_service::SeaService;
use crate::util::{RolePermissions, verify_role_has_permission};

/// Searchable expression. It MUST be identical to the one from the index `idx_audit_logs_search_trgm`
/// (see migration/migration/src/m20261023_072603_add_audit_logs_search_indexes.rss).
///
/// Every nullable column needs `coalesce`, or a NULL might ruin the whole concat otherwise.
const SEARCH_EXPR: &str = "(coalesce(actor_reference, '') || ' ' || coalesce(target_reference, '') || ' ' || coalesce(target_type, '') || ' ' || action_kind || ' ' || coalesce(actor::text, '') || ' ' || coalesce(target::text, '') || ' ' || coalesce(old_payload::text, '') || ' ' || coalesce(new_payload::text, '') || ' ' || coalesce(metadata::text, ''))";

pub struct AuditLogsQuery<'this> {
    connection: &'this SeaService,
}

pub struct AuditLogsQueryParams {
    pub staff_role: Role,
    /// 1-indexed
    pub page: usize,
    pub per_page: usize,
    /// Finds any row having this query somewhere in it.
    pub search_query: Option<String>,
}

#[derive(sqlx::FromRow)]
pub(crate) struct AuditLogRow {
    pub id: i64,
    pub actor_reference: Option<String>,
    pub target_reference: Option<String>,
    pub target_type: Option<String>,
    pub action_kind: String,
    pub happened_at: DateTime<Utc>,
    pub logged_at: DateTime<Utc>,
    pub actor: Option<Value>,
    pub target: Option<Value>,
    pub old_payload: Option<Value>,
    pub new_payload: Option<Value>,
    pub metadata: Option<Value>,
}

fn escape_like(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 2);
    for c in input.chars() {
        if matches!(c, '\\' | '%' | '_') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn push_search_filter(qb: &mut QueryBuilder<Postgres>, pattern: &Option<String>) {
    if let Some(pattern) = pattern {
        qb.push(" WHERE ")
            .push(SEARCH_EXPR)
            .push(" ILIKE ")
            .push_bind(pattern.as_str())
            .push(r" ESCAPE '\'");
    }
}

impl<'this> AuditLogsQuery<'this> {
    pub fn new(connection: &'this SeaService) -> Self {
        Self { connection }
    }

    pub async fn exec(
        &self,
        params: AuditLogsQueryParams,
    ) -> Result<JsonWrappedPaginatedEntity<PresentedLog>, SamambaiaError> {
        if !verify_role_has_permission(&params.staff_role, RolePermissions::ReadAuditLogs) {
            return Err(SamambaiaError::forbidden_err()
                .with_message("Você não tem permissão para visualizar logs de auditoria."));
        }

        let pool = self.connection.db.get_postgres_connection_pool();

        let page = params.page.max(1);
        let per_page = params.per_page.clamp(1, MAX_PER_PAGE);
        let offset = ((page - 1) * per_page) as i64;

        let pattern: Option<String> = params
            .search_query
            .as_deref()
            .map(str::trim)
            .filter(|q| !q.is_empty())
            .map(|q| {
                let q: String = q.chars().take(MAX_SEARCH_LEN).collect();
                format!("%{}%", escape_like(&q))
            });

        let mut count_qb = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM audit_logs");
        push_search_filter(&mut count_qb, &pattern);
        let total: i64 = count_qb.build_query_scalar().fetch_one(pool).await?;

        let rows: Vec<AuditLogRow> = if total == 0 {
            Vec::new()
        } else {
            let mut qb = QueryBuilder::<Postgres>::new(
                "SELECT id, actor_reference, target_reference, target_type, action_kind, \
                        happened_at, logged_at, actor, target, old_payload, new_payload, metadata \
                 FROM audit_logs",
            );
            push_search_filter(&mut qb, &pattern);
            qb.push(" ORDER BY happened_at DESC, id DESC LIMIT ")
                .push_bind(per_page as i64)
                .push(" OFFSET ")
                .push_bind(offset);

            qb.build_query_as::<AuditLogRow>().fetch_all(pool).await?
        };

        let pagination = PaginationResponse::new(page as u32, total as u64, per_page as u32);

        Ok(AuditLogPresenter::to_json_paginated_wrapper(
            rows,
            pagination,
            per_page as u8,
        ))
    }
}
