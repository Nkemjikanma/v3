use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),
    #[error("Invalid environment: {0}")]
    InvalidEnv(String),
}

#[derive(Error, Debug)]
pub enum BookErrors {
    #[error("Error adding new book item: {0}")]
    ErrorAddingBook(String),

    #[error("Error updating book item: {0}")]
    ErrorUpdatingBook(String),

    #[error("Error deleting book item: {0}")]
    ErrorDeletingBook(String),

    #[error("Error fetching books: {0}")]
    ErrorFetchingBooks(String),

    #[error("Book not found: {0}")]
    BookNotFound(String),

    #[error("Something went wrong")]
    BookQueryError(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Books(#[from] BookErrors),

    #[error("Error occured validating payload: {0}")]
    ValidationError(String),

    #[error("Error caused by oversized payload: {0}")]
    OversizedPayloadError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(serde_json::json!({"error": message}))
    }
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Books(BookErrors::ErrorAddingBook(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Books(BookErrors::ErrorFetchingBooks(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Books(BookErrors::BookNotFound(_)) => StatusCode::NOT_FOUND,
            AppError::Books(BookErrors::BookQueryError(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Books(BookErrors::ErrorUpdatingBook(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Books(BookErrors::ErrorDeletingBook(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::OversizedPayloadError(_) => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}

// impl From<ConfigError> for std::io::Error{
//     fn from()
// }
