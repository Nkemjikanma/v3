use crate::config;
use sqlx::PgPool;
pub struct AppState {
    pub app_config: config::Config,
    pub connection: PgPool,
}
