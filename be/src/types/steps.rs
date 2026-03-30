use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Steps {
    pub id: Uuid,
    pub date: NaiveDate,
    pub step_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepsFormData {
    pub step_count: i32,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct StepsQueryInfo {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}
