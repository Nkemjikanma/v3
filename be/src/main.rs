//! main.rs
use be::startup::{create_pool, run};
use be::{
    config::{Config, Environment},
    telemetry::{get_subscriber, init_subscriber},
    types::app::AppState,
};
use std::net::TcpListener;
use std::sync::Arc;
use tracing::info;

type AppError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".to_string())
        .try_into()
        .expect("Failed to parse env");
    let env_file = format!(".env.{}", environment.as_str());
    dotenvy::from_filename(&env_file).ok();

    let subscriber = get_subscriber("be".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = Config::load_config()?;

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address)?;

    info!("Listening here: {:?}", listener);
    // let post = listener.local_addr().unwrap().port();

    let connection = create_pool(&config.database).expect("Failed to connect to Postgres");
    let app_state = Arc::new(AppState {
        app_config: config,
        connection,
    });
    run(listener, app_state)
        .map_err(|e| -> AppError { Box::new(e) })?
        .await
        .map_err(Into::into)
}
