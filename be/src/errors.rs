use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),
}
