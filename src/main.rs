use actix_web::HttpServer;
use dotenvy::dotenv;
use env_logger::{self, Target};
use hubbitos_backend::server::ServerFactory;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();

    HttpServer::new(ServerFactory::exec)
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

