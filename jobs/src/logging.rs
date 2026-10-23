use std::fmt::Display;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, query};

use crate::job::{Job, JobError, JobHandler};

pub const LOGGING_JOB_TYPE: &str = "audit";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TargetType {
    User,
    Article,
    ArticleTag,
    TeamMember,
    FreeBadge,
    Comment,
    Report,
}

impl Display for TargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::User => "User",
            Self::Article => "Article",
            Self::ArticleTag => "ArticleTag",
            Self::TeamMember => "TeamMember",
            Self::FreeBadge => "FreeBadge",
            Self::Comment => "Comment",
            Self::Report => "Report",
        };
        write!(f, "{str}")
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ActionKind {
    Create,
    Delete,
    Edit,
    Access,
}

impl Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Create => "Create",
            Self::Delete => "Delete",
            Self::Edit => "Edit",
            Self::Access => "Access",
        };
        write!(f, "{str}")
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: String,
    pub name: String,
    /// Might repeat both `id` and `name`.
    pub properties: Value,
}

impl EntitySnapshot {
    pub fn new<T: Serialize>(
        id: String,
        name: String,
        properties: T,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id,
            name,
            properties: serde_json::to_value(properties)?,
        })
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum Actor {
    #[default]
    System,
    Entity(EntitySnapshot),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Target {
    pub t_type: TargetType,
    pub entity: EntitySnapshot,
}

#[derive(Builder, Clone, Serialize, Deserialize)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct LoggingJob {
    #[builder(default)]
    pub actor: Actor,
    #[builder(default)]
    pub target: Option<Target>,
    pub action_kind: ActionKind,
    #[builder(default)]
    pub old_payload: Option<Value>,
    #[builder(default)]
    pub new_payload: Option<Value>,
    #[builder(default)]
    pub metadata: Option<Value>,
    #[builder(default = "Utc::now()")]
    pub created_at: DateTime<Utc>,
}

impl LoggingJob {
    pub fn into_job(self) -> Job<Self> {
        Job::new(LOGGING_JOB_TYPE, self)
    }
}

pub struct LoggingHandler {
    // does not need to be a reference or wrapped in Arc since PgPool is already Arc-wrapped
    pool: PgPool,
}

impl LoggingHandler {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pool: pg_pool }
    }
}

#[async_trait]
impl JobHandler<LoggingJob> for LoggingHandler {
    async fn handle(&self, job: &Job<LoggingJob>) -> Result<(), JobError> {
        let (actor_reference, actor_payload) = match &job.payload.actor {
            Actor::System => ("System", None),
            Actor::Entity(entity) => (entity.name.as_str(), Some(&entity.properties)),
        };

        let target_reference = job
            .payload
            .target
            .as_ref()
            .map(|target| format!("{} ({})", target.entity.name, target.t_type));

        let target_payload = job
            .payload
            .target
            .as_ref()
            .map(|target| &target.entity.properties);

        // TODO: make logs repository
        // needs to move infra module to a separate repositories crate
        query(
            r#"
            INSERT INTO audit_logs
            (actor_reference, target_reference, action_kind, happened_at, logged_at,
             actor, target, old_payload, new_payload, metadata)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(actor_reference)
        .bind(target_reference)
        .bind(job.payload.action_kind.to_string())
        .bind(job.payload.created_at)
        .bind(job.meta.scheduled_at.unwrap_or(job.meta.created_at))
        .bind(actor_payload)
        .bind(target_payload)
        .bind(job.payload.old_payload.as_ref())
        .bind(job.payload.new_payload.as_ref())
        .bind(job.payload.metadata.as_ref())
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| JobError::Processing(err.to_string()))
    }
}
