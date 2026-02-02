use crate::types::{
    app::AppState,
    books::{Book, BookCategory, BookFormData, BookStatus},
};
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
use chrono::{Datelike, Utc};
use std::sync::Arc;
use tracing::Instrument;
use uuid::Uuid;

#[tracing::instrument(name = "get_all_books", skip(app_state))]
pub async fn get_all_books(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tracing::instrument(name = "add_book",skip(book, app_state), fields(book_title = %book.title, book_author= %book.author) )]
pub async fn add_book(
    book: web::Json<BookFormData>,
    app_state: web::Data<Arc<AppState>>,
) -> HttpResponse {
    let request_id = uuid::Uuid::new_v4();
    tracing::info!("Adding book to reading list");

    let BookFormData {
        title,
        author,
        status,
        category,
    } = book.into_inner();

    if title.trim().is_empty() {
        tracing::error!(
            "request_id {} - Adding book: title field cannot be empty",
            request_id
        );
        return HttpResponse::BadRequest().body("Title cannot be empty");
    }

    if author.trim().is_empty() {
        tracing::error!(
            "request_id {} - Adding book : author field cannot be empty",
            request_id
        );
        return HttpResponse::BadRequest().body("Author is required");
    }

    let query_span = tracing::info_span!("Adding book : adding book to db",);
    match insert_book(
        title,
        author,
        status,
        category,
        app_state.connection.clone(),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new book detial in database",
    skip(author, title, status, category, pool)
)]
pub async fn insert_book(
    title: String,
    author: String,
    status: BookStatus,
    category: BookCategory,
    pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO books (title, author, status, category, year_read) VALUES ($1, $2, $3, $4, $5)"#,
        title,
        author,
        status as BookStatus,
        category as BookCategory,
        Utc::now().year() as i16,
    )
    .execute(&pool)
    .await.map_err(|e| {
            tracing::error!("Failed to execute query: {:?}",e); 
        });
    Ok(())
}
