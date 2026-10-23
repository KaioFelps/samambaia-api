use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::infra::http::presenters::presenter::PresenterTrait;
use crate::infra::queries::audit_logs::AuditLogRow;

pub struct AuditLogPresenter;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum PresentedLogActor {
    System(String),
    Entity(Value),
}

#[derive(Debug, Serialize)]
pub struct PresentedTarget {
    r#type: String,
    properties: Value,
}

#[derive(Debug, Serialize)]
pub struct PresentedLog {
    id: i64,
    #[serde(rename = "actorReference")]
    actor_reference: Option<String>,
    #[serde(rename = "targetReference")]
    target_reference: Option<String>,
    #[serde(rename = "actionKind")]
    action_kind: String,
    #[serde(rename = "happenedAt")]
    happened_at: DateTime<Utc>,
    #[serde(rename = "loggedAt")]
    logged_at: DateTime<Utc>,
    actor: PresentedLogActor,
    target: Option<PresentedTarget>,
    #[serde(rename = "oldPayload")]
    old_payload: Option<Value>,
    #[serde(rename = "newPayload")]
    new_payload: Option<Value>,
    metadata: Option<Value>,
}

impl PresenterTrait<AuditLogRow, PresentedLog> for AuditLogPresenter {
    fn to_http(row: AuditLogRow) -> PresentedLog {
        let actor = present_actor(row.actor_reference.as_deref(), row.actor);

        let target = row.target_type.map(|t_type| PresentedTarget {
            r#type: t_type,
            properties: row.target.unwrap_or(Value::Null),
        });

        PresentedLog {
            id: row.id,
            actor_reference: row.actor_reference,
            target_reference: row.target_reference,
            action_kind: row.action_kind,
            happened_at: row.happened_at,
            logged_at: row.logged_at,
            actor,
            target,
            old_payload: row.old_payload,
            new_payload: row.new_payload,
            metadata: row.metadata,
        }
    }

    fn to_json_paginated_wrapper(
        entities: Vec<AuditLogRow>,
        pagination: crate::core::pagination::PaginationResponse,
        per_page: u8,
    ) -> super::presenter::JsonWrappedPaginatedEntity<PresentedLog> {
        let data = entities.into_iter().map(Self::to_http).collect::<Vec<_>>();
        let pagination = super::pagination::PaginationPresenter::to_http(pagination, per_page);

        super::presenter::JsonWrappedPaginatedEntity { pagination, data }
    }
}

fn present_actor(reference: Option<&str>, actor: Option<Value>) -> PresentedLogActor {
    match actor {
        Some(properties) => PresentedLogActor::Entity(properties),
        None => PresentedLogActor::System(reference.unwrap_or("System").to_owned()),
    }
}
