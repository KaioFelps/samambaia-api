use actix_web::{web::Data, HttpServer};
use dotenvy::dotenv;
use env_logger::{self, Target};
use log::{error, info};
use migration::{Migrator, MigratorTrait};
use samambaia::{
    configs::inertia::initialize_inertia, env_config::RustEnv, infra::sea::sea_service::SeaService,
    server::ServerFactory, ENV_VARS,
};

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

    let migration_result = Migrator::up(&sea_service.db, None).await;

    if migration_result.is_err() {
        let err = migration_result.unwrap_err();
        error!("Error occurred on applying pending migrations: \n{}", err);
    }

    let sea_service = actix_web::web::Data::new(sea_service);

    let inertia = initialize_inertia().await?;
    let inertia = Data::new(inertia);
    let inertia_data = inertia.clone();

    let server = HttpServer::new(move || {
        ServerFactory::exec_with_sea(sea_service.clone()).app_data(inertia_data.clone())
    })
    .bind((ENV_VARS.host.as_str(), ENV_VARS.port))?
    .workers(ENV_VARS.workers);

    let is_production = ENV_VARS.rust_env == RustEnv::Production;
    let mut node_process = None;

    if is_production {
        node_process = Some(inertia.start_node_server("dist/ssr/ssr.js".into())?);
    }

    let server = server.run().await;

    if is_production {
        if let Some(node_process) = node_process {
            node_process.kill().await?;
            info!("Inertia SSR server has shutdown.");
        }
    }

    info!("Shutting down the application.");
    server
}
