use crate::{
    config,
    errors::AppError,
    routes::{
        books::configure_books, check::greet, songs::configure_songs, steps::configure_steps,
    },
    types::app::AppState,
};

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::dev::Server;
use actix_web::middleware::NormalizePath;
use actix_web::{
    error::{self, ResponseError},
    http, web, App, HttpServer,
};
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
                let api_error = match &err {
                    error::JsonPayloadError::Overflow { limit } => {
                        tracing::error!("Over sized JSON payload being sent in: {limit}",);
                        AppError::OversizedPayloadError(err.to_string())
                    }

                    _ => {
                        tracing::error!("JSON extraction error: {}", err);
                        AppError::ValidationError(err.to_string())
                    }
                };

                error::InternalError::from_response(err, api_error.error_response()).into()
            });

        let governor_conf = GovernorConfigBuilder::default()
            .seconds_per_request(1)
            .burst_size(60)
            .finish()
            .unwrap();

        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("https://nkem.dev")
                    .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(Governor::new(&governor_conf))
            .wrap(NormalizePath::trim())
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .configure(configure_books)
                    .configure(configure_songs)
                    .configure(configure_steps)
                    .route("", web::get().to(greet)),
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

// - typical ordering of middlewares
// App::new()
//     .wrap(actix_cors::Cors::default())
//     .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(...))
//     .wrap(actix_session::SessionMiddleware::new(...))
//     .wrap(actix_identity::IdentityMiddleware::default())
//     .wrap(actix_limitation::RateLimiter::new(...))
