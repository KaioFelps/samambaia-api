use std::io;

use actix_web::web::{Data, ThinData};
use actix_web::{HttpServer, middleware};
use dotenvy::dotenv;
use env_logger::{self, Target};
use jobs::logging::LoggingHandler;
use jobs::queue::JobQueue;
use jobs::worker::WorkerConfig;
use log::{error, info};
use migration::{Migrator, MigratorTrait};
use redis_pool::RedisPoolTlsFiles;
use samambaia::configs::app::APP_CONFIG;
use samambaia::configs::env::RustEnv;
use samambaia::configs::inertia::initialize_inertia;
use samambaia::configs::redis_sessions::PooledRedisSessionStore;
use samambaia::infra::sea::sea_service::SeaService;
use samambaia::server::ServerFactory;
use tokio::sync::broadcast;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::new()
        .parse_env("RUST_LOG")
        .target(Target::Stdout)
        .init();

    let sea_service = match SeaService::new().await {
        Err(err) => panic!("Sea ORM should be able to connect to the database: {}", err),
        Ok(conn) => conn,
    };

    let redis_pool = redis_pool::build_redis_pool(
        APP_CONFIG.redis_url,
        RedisPoolTlsFiles::try_from_paths(
            APP_CONFIG.redis_client_cert_path,
            APP_CONFIG.redis_client_key_path,
            APP_CONFIG.redis_root_cert_path,
        ),
    )
    .map_err(|e| io::Error::other(e.to_string()))?;

    let job_queue = ThinData(
        JobQueue::new(redis_pool.clone())
            .await
            .with_visibility_timeout(chrono::Duration::minutes(10)),
    );

    let migration_result = Migrator::up(&sea_service.db, None).await;
    if let Err(err) = migration_result {
        error!("Error occurred on applying pending migrations: \n{}", err);
    }

    let sea_service = actix_web::web::Data::new(sea_service);
    let sea_service_clone = sea_service.clone();

    let (shutdown_tx, _) = broadcast::channel(1);

    let worker_queue = (*job_queue).clone();
    let worker_shutdown = shutdown_tx.subscribe();
    let pg_pool = sea_service.db.get_postgres_connection_pool().clone();
    let worker_handle = tokio::spawn(async move {
        let handler = LoggingHandler::new(pg_pool.clone());
        let config = WorkerConfig {
            concurrency: APP_CONFIG.job_queue_workers,
            ..Default::default()
        };

        let worker = jobs::worker::Worker::new(worker_queue, handler, config, worker_shutdown);
        worker.run().await;
    });

    let inertia = initialize_inertia().await?;
    let inertia = Data::new(inertia);
    let inertia_clone = inertia.clone();

    let server = HttpServer::new(move || {
        ServerFactory::exec_with_sea(
            sea_service_clone.clone(),
            PooledRedisSessionStore::new(redis_pool.clone()),
        )
        .app_data(inertia_clone.clone())
        .app_data(job_queue.clone())
        .wrap(middleware::Logger::default())
    })
    .bind((APP_CONFIG.host, APP_CONFIG.port))?
    .workers(APP_CONFIG.workers);

    let is_production = APP_CONFIG.rust_env == RustEnv::Production;
    let mut node_process = None;

    if is_production {
        node_process = Some(inertia.start_node_server("dist/ssr/ssr.js".into())?);
    }

    let server = server.run().await;

    if is_production && let Some(node_process) = node_process {
        node_process.kill().await?;
        info!("Inertia SSR server has shutdown.");
    }

    let _ = shutdown_tx.send(());
    worker_handle.await?;

    sea_service.db.get_postgres_connection_pool().close().await;

    info!("Shutting down the application.");
    server
}
