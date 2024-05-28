use actix_web::HttpServer;
use dotenvy::dotenv;
use env_logger::{self, Target};
use hubbitos_backend::{server::ServerFactory, ENV_VARS};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();

    HttpServer::new(ServerFactory::exec)
    .bind((ENV_VARS.host.as_str(), ENV_VARS.port))?
    .workers(ENV_VARS.workers)
    .run()
    .await
}

