use std::collections::HashMap;

use actix_session::storage::{
    LoadError,
    SaveError,
    SessionKey,
    SessionStore,
    UpdateError,
    generate_session_key,
};
use actix_web::cookie::time::Duration;
use redis::AsyncCommands;
use redis_pool::RedisPool;

type SessionState = HashMap<String, String>;
const SESSIONS_PREFIX: &str = "session:";

#[derive(Clone)]
pub struct PooledRedisSessionStore {
    pool: RedisPool,
}

impl PooledRedisSessionStore {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    fn cache_key(&self, key: &SessionKey) -> String {
        format!("{SESSIONS_PREFIX}{}", key.as_ref())
    }
}

impl SessionStore for PooledRedisSessionStore {
    async fn load(&self, key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|e| LoadError::Other(e.into()))?;

        let value: Option<String> = connection
            .get(self.cache_key(key))
            .await
            .map_err(|e| LoadError::Other(e.into()))?;

        match value {
            None => Ok(None),
            Some(v) => serde_json::from_str(&v)
                .map(Some)
                .map_err(|e| LoadError::Deserialization(e.into())),
        }
    }

    async fn save(&self, state: SessionState, ttl: &Duration) -> Result<SessionKey, SaveError> {
        let body = serde_json::to_string(&state).map_err(|e| SaveError::Serialization(e.into()))?;
        let key = generate_session_key();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SaveError::Other(e.into()))?;

        let _: () = conn
            .set_ex(self.cache_key(&key), body, ttl.whole_seconds() as u64)
            .await
            .map_err(|e| SaveError::Other(e.into()))?;

        Ok(key)
    }

    async fn update(
        &self,
        key: SessionKey,
        state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let body =
            serde_json::to_string(&state).map_err(|e| UpdateError::Serialization(e.into()))?;

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| UpdateError::Other(e.into()))?;
        let _: () = conn
            .set_ex(self.cache_key(&key), body, ttl.whole_seconds() as u64)
            .await
            .map_err(|e| UpdateError::Other(e.into()))?;

        Ok(key)
    }

    async fn update_ttl(&self, key: &SessionKey, ttl: &Duration) -> Result<(), anyhow::Error> {
        let mut conn = self.pool.get().await?;
        let _: () = conn
            .expire(self.cache_key(key), ttl.whole_seconds())
            .await?;
        Ok(())
    }

    async fn delete(&self, key: &SessionKey) -> Result<(), anyhow::Error> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.del(self.cache_key(key)).await?;
        Ok(())
    }
}
