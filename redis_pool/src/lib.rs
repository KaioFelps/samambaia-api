use std::path::Path;

use deadpool::managed::{self, Metrics, RecycleError, RecycleResult};
use redis::aio::MultiplexedConnection;
use redis::{ClientTlsConfig, RedisError, TlsCertificates};

pub struct TlsManager {
    client: redis::Client,
}

impl managed::Manager for TlsManager {
    type Type = MultiplexedConnection;
    type Error = RedisError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.client.get_multiplexed_async_connection().await
    }

    async fn recycle(&self, conn: &mut Self::Type, _: &Metrics) -> RecycleResult<Self::Error> {
        redis::cmd("PING")
            .query_async::<String>(conn)
            .await
            .map(|_| ())
            .map_err(RecycleError::Backend)
    }
}

pub type RedisPool = managed::Pool<TlsManager>;
pub type RedisConnection = managed::Object<TlsManager>;
pub type PoolError = managed::PoolError<RedisError>;

#[derive(Clone)]
pub struct RedisPoolTlsFiles<'this> {
    client_cert_path: &'this Path,
    client_key_path: &'this Path,
    root_cert_path: &'this Path,
}

impl<'this> RedisPoolTlsFiles<'this> {
    pub fn try_from_paths(
        client_cert_path: Option<&'this str>,
        client_key_path: Option<&'this str>,
        root_cert_path: Option<&'this str>,
    ) -> Option<Self> {
        if let Some(client_cert_path) = client_cert_path
            && let Some(client_key_path) = client_key_path
            && let Some(root_cert_path) = root_cert_path
        {
            Some(Self {
                client_cert_path: Path::new(client_cert_path),
                client_key_path: Path::new(client_key_path),
                root_cert_path: Path::new(root_cert_path),
            })
        } else {
            None
        }
    }
}

impl<'this> IntoIterator for RedisPoolTlsFiles<'this> {
    type Item = &'this Path;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.client_cert_path,
            self.client_key_path,
            self.root_cert_path,
        ]
        .into_iter()
    }
}

/// Creates a Redis connection pool.
///
/// If any of certs or the private key is not present, defaults to not using TLS.
pub fn build_redis_pool(
    url: &str,
    tls_files: Option<RedisPoolTlsFiles<'_>>,
) -> Result<RedisPool, Box<dyn std::error::Error>> {
    let client = if let Some(paths) = &tls_files
        && paths.clone().into_iter().all(|p| p.is_file())
    {
        let certs = TlsCertificates {
            client_tls: Some(ClientTlsConfig {
                client_cert: std::fs::read(paths.client_cert_path)?,
                client_key: std::fs::read(paths.client_key_path)?,
            }),
            root_cert: Some(std::fs::read(paths.root_cert_path)?),
        };
        redis::Client::build_with_tls(url, certs)?
    } else {
        if let Some(paths) = tls_files
            && paths.clone().into_iter().any(|p| p.is_file())
        {
            log::warn!(
                "Certificados do Redis incompletos, conectando sem mTLS (cert: {}, key: {}, ca: {})",
                paths.client_cert_path.is_file(),
                paths.client_key_path.is_file(),
                paths.root_cert_path.is_file(),
            );
        }
        redis::Client::open(url)?
    };

    Ok(RedisPool::builder(TlsManager { client })
        .max_size(16)
        .build()?)
}
