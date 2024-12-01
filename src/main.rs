use actix_web::HttpServer;
use dotenvy::dotenv;
use env_logger::{self, Target};
use hubbitos_backend::{infra::sea::sea_service::SeaService, server::ServerFactory, ENV_VARS};
use log::error;
use migration::{Migrator, MigratorTrait};

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

        error!("Error occurred on applying pending migrations: \n{}\n", err);
    }

    let sea_service = actix_web::web::Data::new(sea_service);

    HttpServer::new(move || ServerFactory::exec_with_sea(sea_service.clone()))
        .bind((ENV_VARS.host.as_str(), ENV_VARS.port))?
        .workers(ENV_VARS.workers)
        .run()
        .await
}
