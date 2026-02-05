use crate::errors::ConfigError;
use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DBConfig,
    pub application: ApplicatioonSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DBConfig {
    pub username: String,
    pub password: SecretString,
    pub port: String,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicatioonSettings {
    pub port: String,
    pub host: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Local,
    Production,
}

impl Config {
    pub fn load_config() -> Result<Self, ConfigError> {
        let application = ApplicatioonSettings {
            port: std::env::var("APP_PORT")
                .map_err(|_| {
                    tracing::warn!("Using default PORT: 8000");
                    ConfigError::MissingEnv("APP_PORT".to_string())
                })
                .unwrap_or_else(|_| "8000".to_string()),
            host: std::env::var("APP_HOST")
                .map_err(|_| {
                    tracing::warn!("Using default HOST: 127.0.0.1");
                    ConfigError::MissingEnv("APP_HOST".to_string())
                })
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
        };
        let database = DBConfig {
            username: std::env::var("DB_USERNAME")
                .map_err(|_| {
                    tracing::warn!("Using default username for db");
                    ConfigError::MissingEnv("DB_USERNAME".to_string())
                })
                .unwrap_or_else(|_| "postgres".to_string()),
            password: SecretString::from(
                std::env::var("DB_PASSWORD")
                    .map_err(|_| {
                        tracing::warn!("Using default password for db");
                        ConfigError::MissingEnv("DB_PASSWORD".to_string())
                    })
                    .unwrap_or_else(|_| "password".to_string()),
            ),
            port: std::env::var("DB_PORT")
                .map_err(|_| {
                    tracing::warn!("Using default port for db");
                    ConfigError::MissingEnv("DB_POSRT".to_string())
                })
                .unwrap_or_else(|_| "5432".to_string()),

            host: std::env::var("DB_HOST")
                .map_err(|_| {
                    tracing::warn!("Using default host");
                    ConfigError::MissingEnv("DB_HOST".to_string())
                })
                .unwrap_or_else(|_| "127.0.0.1".to_string()),

            database_name: std::env::var("DB_NAME")
                .map_err(|_| {
                    tracing::warn!("Using default db names");
                    ConfigError::MissingEnv("DB_NAME".to_string())
                })
                .unwrap_or_else(|_| "coco_core".to_string()),
        };
        Ok(Self {
            application,
            database,
        })
    }
}

impl DBConfig {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" | "production" => Ok(Self::Production),
            other => Err(format!("Unknown environment: {}", other)),
        }
    }
}
