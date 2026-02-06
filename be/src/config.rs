use crate::errors::ConfigError;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DBConfig,
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DBConfig {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool, // determine if we demand the connection to be encrypted or not
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Local,
    Production,
}

impl Config {
    pub fn load_config() -> Result<Self, ConfigError> {
        let env: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "local".to_string())
            .try_into()
            .map_err(|e: String| ConfigError::InvalidEnv(e))?;

        match env {
            Environment::Local => Self::local_config(),
            Environment::Production => Self::production_config(),
        }
    }

    fn production_config() -> Result<Self, ConfigError> {
        let application = ApplicationSettings {
            port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .map_err(|_| {
                    ConfigError::InvalidEnv("APP_PORT must be valid port number".to_string())
                })?,
            host: "0.0.0.0".to_string(),
        };

        let database = DBConfig {
            username: std::env::var("DB_USERNAME")
                .map_err(|_| ConfigError::MissingEnv("DB_USERNAME".to_string()))?,
            password: SecretString::from(
                std::env::var("DB_PASSWORD")
                    .map_err(|_| ConfigError::MissingEnv("DB_PASSWORD".to_string()))?,
            ),
            port: std::env::var("DB_PORT")
                .map_err(|_| ConfigError::MissingEnv("DB_PORT".to_string()))?
                .parse()
                .map_err(|_| {
                    ConfigError::InvalidEnv("DB_PORT must be a valid number".to_string())
                })?,
            host: std::env::var("DB_HOST")
                .map_err(|_| ConfigError::MissingEnv("DB_HOST".to_string()))?,
            database_name: std::env::var("DB_NAME")
                .map_err(|_| ConfigError::MissingEnv("DB_NAME".to_string()))?,
            require_ssl: true,
        };

        Ok(Self {
            application,
            database,
        })
    }

    fn local_config() -> Result<Self, ConfigError> {
        let application = ApplicationSettings {
            port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| {
                    tracing::warn!("Using default PORT: 8000");
                    "8000".to_string()
                })
                .parse()
                .unwrap_or(8000),
            host: std::env::var("APP_HOST").unwrap_or_else(|_| {
                tracing::warn!("Using default HOST: 127.0.0.1");
                "127.0.0.1".to_string()
            }),
        };

        let database = DBConfig {
            username: std::env::var("DB_USERNAME").unwrap_or_else(|_| {
                tracing::warn!("Using default username for db");
                "postgres".to_string()
            }),
            password: SecretString::from(std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
                tracing::warn!("Using default password for db");
                "password".to_string()
            })),
            port: std::env::var("DB_PORT")
                .unwrap_or_else(|_| {
                    tracing::warn!("Using default port for db");
                    "5432".to_string()
                })
                .parse()
                .unwrap_or(5432),
            host: std::env::var("DB_HOST").unwrap_or_else(|_| {
                tracing::warn!("Using default db host");
                "127.0.0.1".to_string()
            }),
            database_name: std::env::var("DB_NAME").unwrap_or_else(|_| {
                tracing::warn!("Using default db name");
                "coco_core".to_string()
            }),
            require_ssl: false,
        };

        Ok(Self {
            application,
            database,
        })
    }
}

impl DBConfig {
    pub fn connection_options(&self) -> PgConnectOptions {
        // SecretString::from(format!(
        //     "postgres://{}:{}@{}:{}/{}",
        //     self.username,
        //     self.password.expose_secret(),
        //     self.host,
        //     self.port,
        //     self.database_name
        // ))
        //
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer // try encrypted connection, fallback to unencrypted if it fails
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
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
