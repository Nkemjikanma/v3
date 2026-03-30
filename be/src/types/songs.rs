use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

use crate::common::valid_string_entry::ValidStringEntry;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub artist: String,
    pub instrument: Instrument,
    pub started_learning_at: chrono::NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "instrument", rename_all = "lowercase")]
pub enum Instrument {
    Guitar,
    Piano,
    Both,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongFormData {
    pub title: ValidStringEntry,
    pub artist: ValidStringEntry,
    pub instrument: Instrument,
    pub started_learning_at: chrono::NaiveDate,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSongFormData {
    pub title: Option<ValidStringEntry>,
    pub artist: Option<ValidStringEntry>,
    pub instrument: Option<Instrument>,
    pub notes: Option<String>,
    pub started_learning_at: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct SongQueryInfo {
    pub instrument: Option<Instrument>,
}
