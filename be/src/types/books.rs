use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub status: BookStatus,
    pub category: BookCategory,
    pub year_read: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "book_status", rename_all = "lowercase")]
pub enum BookStatus {
    Reading,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "book_category", rename_all = "lowercase")]
pub enum BookCategory {
    Technical,
    Leisure,
    Music,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookFormData {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
    pub category: BookCategory,
}
