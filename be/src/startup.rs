use crate::{
    config,
    routes::{
        books::{add_book, get_all_books},
        check::greet,
        songs::songs,
    },
    types::app::AppState,
};
use actix_web::dev::Server;
use actix_web::middleware::NormalizePath;
use actix_web::{error, web, App, HttpResponse, HttpServer};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use tracing_actix_web::TracingLogger;

#[tracing::instrument(name = "run", skip_all)]
pub fn run(listener: TcpListener, app_state: Arc<AppState>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(app_state);
    let server = HttpServer::new(move || {
        // Set payload limit to 4kb
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                tracing::error!("Over sized JSON payload being sent in");
                error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            });

        App::new()
            .wrap(NormalizePath::trim())
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .route("", web::get().to(greet))
                    .route("/books", web::get().to(get_all_books))
                    .route("/books", web::post().to(add_book))
                    .route("/songs", web::get().to(songs)),
            )
            .app_data(connection.clone())
            .app_data(json_config)
    })
    // .bind(listener)?
    .listen(listener)?
    .run();

    Ok(server)
}

#[tracing::instrument(name = "pool", skip_all)]
pub fn create_pool(config: &config::DBConfig) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Creating db pool");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(200))
        .idle_timeout(Duration::from_secs(300))
        .connect_lazy_with(config.connection_options());

    tracing::info!("Database pool connection created");
    Ok(pool)
}
