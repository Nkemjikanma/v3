//! main.rs
use be::config::Config;
use be::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = Config::load_config()?;

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address)?;

    info!("Listening here: {:?}", listener);
    // let post = listener.local_addr().unwrap().port();

    run(listener)?.await
}
