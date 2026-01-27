use crate::errors;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub database: DBConfig,
    pub application_port: String,
}

#[derive(Deserialize, Clone)]
pub struct DBConfig {
    pub username: String,
    pub password: String,
    pub port: String,
    pub host: String,
    pub database_name: String,
}

impl Config {
    pub fn load_config() -> Result<Self, errors::ConfigError> {
        Ok(Self {
            application_port: std::env::var("APP_PORT").unwrap_or_else(|_| {
                tracing::warn!("Using default PORT: 8000");
                8000.to_string()
            }),
            database: DBConfig {
                username: std::env::var("DB_USERNAME").unwrap_or_else(|_| {
                    tracing::warn!("Using default username for db");
                    "postgres".to_string()
                }),
                password: std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
                    tracing::warn!("Using default password for db");
                    "password".to_string()
                }),
                port: std::env::var("DB_PORT").unwrap_or_else(|_| {
                    tracing::warn!("Using default port for db");
                    "5432".to_string()
                }),

                host: std::env::var("DB_HOST").unwrap_or_else(|_| {
                    tracing::warn!("Using default host address for db");
                    "127.0.0.1".to_string()
                }),

                database_name: std::env::var("DB_NAME").unwrap_or_else(|_| {
                    tracing::warn!("Using default name for db");
                    "coco_core".to_string()
                }),
            },
        })
    }
}

impl DBConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
