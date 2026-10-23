use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct JobMetadata {
    pub(crate) id: Uuid,
    pub(crate) job_type: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) scheduled_at: Option<DateTime<Utc>>,
    pub(crate) attempts: u32,
    pub(crate) max_attempts: u32,
    pub(crate) last_error: Option<String>,
}

impl JobMetadata {
    pub fn new(job_type: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            job_type: job_type.to_string(),
            created_at: Utc::now(),
            scheduled_at: None,
            attempts: 0,
            max_attempts: 3,
            last_error: None,
        }
    }

    pub fn with_max_attempts(mut self, max: u32) -> Self {
        self.max_attempts = max;
        self
    }

    pub fn with_delay(mut self, delay: chrono::Duration) -> Self {
        self.scheduled_at = Some(Utc::now() + delay);
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Job<T> {
    pub(crate) meta: JobMetadata,
    pub(crate) payload: T,
}

impl<'this, T: Serialize + Deserialize<'this> + Clone> Job<T> {
    pub fn new(job_type: &'this str, payload: T) -> Self {
        Self {
            meta: JobMetadata::new(job_type),
            payload,
        }
    }

    pub fn with_max_attempts(mut self, max: u32) -> Self {
        self.meta = self.meta.with_max_attempts(max);
        self
    }

    pub fn with_delay(mut self, delay: chrono::Duration) -> Self {
        self.meta = self.meta.with_delay(delay);
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JobError {
    #[error("Processing failed: {0}")]
    Processing(String),

    #[error("Transient error: {0}")]
    Transient(String),
}

#[async_trait]
pub trait JobHandler<JobT>: Send + Sync {
    async fn handle(&self, job: &Job<JobT>) -> Result<(), JobError>;
}
