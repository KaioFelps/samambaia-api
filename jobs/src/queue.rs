use std::sync::LazyLock;

use chrono::{Duration, Utc};
use indoc::indoc;
use redis::{AsyncCommands, RedisError, Script};
use redis_pool::{PoolError, RedisConnection, RedisPool};
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::job::Job;

/// IDs of jobs ready to be executed
const READY_QUEUE: &str = "jobs:ready";
/// IDs of jobs that are sent to be executed. Maps to a deadline (ms) that defines when the job
/// should go back to the execution queue.
const INFLIGHT_SET: &str = "jobs:inflight";
/// IDs of jobs that must be delayed to be executed (mapping to the delay (ms)).
const DELAYED_SET: &str = "jobs:delayed";
/// IDs that has exceeded the retries.
const DEAD_LETTER_QUEUE: &str = "jobs:dead";
/// The payload of the jobs mapped by their IDs.
const PAYLOADS: &str = "jobs:payloads";

/// How many time the job can be in execution before considered as lost.
/// MUST be longer than worker's `job_timeout`
const DEFAULT_VISIBILITY_TIMEOUT_SECS: i64 = 900;

/// The maximum amount of jobs to be moved per call
const PROMOTE_BATCH: usize = 100;

/// Non-blocking script to atomically:
/// - realloc expired scheduled jobs
/// - send expired in-execution jobs back to the execution queue
/// - mark the next job as in-execution
static DEQUEUE_SCRIPT: LazyLock<Script> = LazyLock::new(|| {
    Script::new(indoc! {"
        local now = ARGV[1]
        local deadline = ARGV[2]
        local batch = tonumber(ARGV[3])

        local due = redis.call('ZRANGEBYSCORE', KEYS[3], '-inf', now, 'LIMIT', 0, batch)
        for _, id in ipairs(due) do
        redis.call('ZREM', KEYS[3], id)
        redis.call('RPUSH', KEYS[1], id)
        end

        local expired = redis.call('ZRANGEBYSCORE', KEYS[2], '-inf', now, 'LIMIT', 0, batch)
        for _, id in ipairs(expired) do
        redis.call('ZREM', KEYS[2], id)
        redis.call('RPUSH', KEYS[1], id)
        end

        local id = redis.call('LPOP', KEYS[1])
        if not id then
        return nil
        end

        local payload = redis.call('HGET', KEYS[4], id)
        if not payload then
        return nil
        end

        redis.call('ZADD', KEYS[2], deadline, id)
        return {id, payload}
    "})
});

#[derive(Clone)]
pub struct JobQueue {
    pool: RedisPool,
    visibility_timeout: Duration,
}

impl JobQueue {
    pub async fn new(pool: RedisPool) -> Self {
        Self {
            pool,
            visibility_timeout: Duration::seconds(DEFAULT_VISIBILITY_TIMEOUT_SECS),
        }
    }

    /// Defines for how long a job can be in execution before being moved back to the
    /// _ready_ queue.
    /// **Must** be longer than [`WorkerConfig::job_timeout`](super::worker::WorkerConfig::job_timeout).
    pub fn with_visibility_timeout(mut self, timeout: Duration) -> Self {
        self.visibility_timeout = timeout;
        self
    }

    async fn get_conn(&self) -> Result<RedisConnection, QueueError> {
        self.pool.get().await.map_err(QueueError::RedisPool)
    }

    /// Enqueue a job (immediately, or at `scheduled_at` if set)
    pub async fn enqueue<T: Serialize>(&self, job: Job<T>) -> Result<Uuid, QueueError> {
        let mut conn = self.get_conn().await?;

        let job_id = job.meta.id;
        let id = job_id.to_string();
        let job_json = serde_json::to_string(&job)?;

        let mut pipe = redis::pipe();
        pipe.atomic().hset(PAYLOADS, &id, &job_json).ignore();

        match job.meta.scheduled_at {
            Some(at) => {
                pipe.zadd(DELAYED_SET, &id, at.timestamp_millis()).ignore();
            }
            None => {
                pipe.rpush(READY_QUEUE, &id).ignore();
            }
        }

        let _: () = pipe.query_async(&mut *conn).await?;

        Ok(job_id)
    }

    /// Takes the next available job (in a non-blocking fashion).
    /// It's up to the callee to sleep before retrying.
    pub async fn dequeue<T: DeserializeOwned>(&self) -> Result<Option<Job<T>>, QueueError> {
        let mut conn = self.get_conn().await?;

        let now = Utc::now().timestamp_millis();
        let deadline = now + self.visibility_timeout.num_milliseconds();

        let popped: Option<(String, String)> = DEQUEUE_SCRIPT
            .key(READY_QUEUE)
            .key(INFLIGHT_SET)
            .key(DELAYED_SET)
            .key(PAYLOADS)
            .arg(now)
            .arg(deadline)
            .arg(PROMOTE_BATCH)
            .invoke_async(&mut *conn)
            .await?;

        let Some((id, payload)) = popped else {
            return Ok(None);
        };

        match serde_json::from_str::<Job<T>>(&payload) {
            Ok(job) => Ok(Some(job)),
            Err(e) => {
                let mut pipe = redis::pipe();
                pipe.atomic()
                    .zrem(INFLIGHT_SET, &id)
                    .ignore()
                    .rpush(DEAD_LETTER_QUEUE, &id)
                    .ignore();
                let _: () = pipe.query_async(&mut *conn).await?;

                Err(QueueError::Serialization(e))
            }
        }
    }

    /// Mark job as completed
    pub async fn complete(&self, job_id: Uuid) -> Result<(), QueueError> {
        let mut conn = self.get_conn().await?;
        let id = job_id.to_string();

        let mut pipe = redis::pipe();
        pipe.atomic()
            .zrem(INFLIGHT_SET, &id)
            .ignore()
            .hdel(PAYLOADS, &id)
            .ignore();
        let _: () = pipe.query_async(&mut *conn).await?;

        Ok(())
    }

    /// Mark job as failed and handle retry (with exponential backoff) or dead letter
    pub async fn fail<T: Serialize>(&self, mut job: Job<T>, error: &str) -> Result<(), QueueError> {
        let mut conn = self.get_conn().await?;
        let id = job.meta.id.to_string();

        job.meta.attempts += 1;
        job.meta.last_error = Some(error.to_string());

        let mut pipe = redis::pipe();
        pipe.atomic();

        if job.meta.attempts < job.meta.max_attempts {
            let delay_seconds = 2_i64.pow(job.meta.attempts.min(10));
            let run_at = Utc::now() + Duration::seconds(delay_seconds);
            job.meta.scheduled_at = Some(run_at);

            let job_json = serde_json::to_string(&job)?;
            pipe.hset(PAYLOADS, &id, &job_json)
                .ignore()
                .zrem(INFLIGHT_SET, &id)
                .ignore()
                .zadd(DELAYED_SET, &id, run_at.timestamp_millis())
                .ignore();
        } else {
            let job_json = serde_json::to_string(&job)?;
            pipe.hset(PAYLOADS, &id, &job_json)
                .ignore()
                .zrem(INFLIGHT_SET, &id)
                .ignore()
                .rpush(DEAD_LETTER_QUEUE, &id)
                .ignore();
        }

        let _: () = pipe.query_async(&mut *conn).await?;

        Ok(())
    }

    /// Get queue statistics
    pub async fn stats(&self) -> Result<QueueStats, QueueError> {
        let mut conn = self.get_conn().await?;

        let pending: i64 = conn.llen(READY_QUEUE).await?;
        let processing: i64 = conn.zcard(INFLIGHT_SET).await?;
        let scheduled: i64 = conn.zcard(DELAYED_SET).await?;
        let dead: i64 = conn.llen(DEAD_LETTER_QUEUE).await?;

        Ok(QueueStats {
            pending: pending as u64,
            processing: processing as u64,
            scheduled: scheduled as u64,
            dead: dead as u64,
        })
    }
}

#[derive(Debug)]
pub struct QueueStats {
    pub pending: u64,
    pub processing: u64,
    pub scheduled: u64,
    pub dead: u64,
}

#[derive(Debug)]
pub enum QueueError {
    Redis(redis::RedisError),
    RedisPool(PoolError),
    Serialization(serde_json::Error),
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Redis(e) => write!(f, "redis error: {e}"),
            Self::RedisPool(e) => write!(f, "redis pool error: {e}"),
            Self::Serialization(e) => write!(f, "serialization error: {e}"),
        }
    }
}

impl std::error::Error for QueueError {}

impl From<serde_json::Error> for QueueError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

impl From<RedisError> for QueueError {
    fn from(value: RedisError) -> Self {
        Self::Redis(value)
    }
}
