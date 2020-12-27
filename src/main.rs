#[macro_use]
extern crate log;

mod application;
mod domain;
mod error;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.sample");

    env_logger::init();

    let server = presentation::http_server::server::Server::new(3000);

    server.run().await;

    Ok(())
}
