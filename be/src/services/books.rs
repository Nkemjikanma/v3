use crate::common::valid_string_entry::ValidStringEntry;
use crate::errors::{AppError, BookErrors};
use crate::types::books::{
    Book, BookCategory, BookFormData, BookQueryInfo, BookStatus, UpdateBookFormData,
};
use chrono::{Datelike, Utc};
use uuid::Uuid;

pub struct BookService;

impl BookService {
    #[tracing::instrument(name = "get_book_by_id", skip(pool))]
    pub async fn get_book_by_id(book_id: Uuid, pool: &sqlx::PgPool) -> Result<Book, AppError> {
        let book = sqlx::query_as!(Book, r#"SELECT id, title, author, status AS "status: BookStatus", category AS "category: BookCategory", year_read, created_at, updated_at FROM books WHERE id = $1"#, book_id).fetch_one(pool)
            .await
            .map_err(|e: sqlx::Error| {
                tracing::error!("Failed to fetch book with id, {}. {:?}", book_id, e);

                match e {
                    sqlx::Error::RowNotFound => BookErrors::BookNotFound(book_id.to_string()),
                    _ => BookErrors::BookQueryError("Error fetching book".to_string())
                }
            })?;

        Ok(book)
    }

    #[tracing::instrument(name = "update book", skip(pool))]
    pub async fn update_book(
        book_id: Uuid,
        update_body: &UpdateBookFormData,
        pool: &sqlx::PgPool,
    ) -> Result<(), AppError> {
        let old_book = sqlx::query_as!(Book, r#"SELECT id, title, author, status AS "status: BookStatus", category AS "category: BookCategory", year_read, created_at, updated_at FROM books WHERE id = $1"#, book_id)
            .fetch_one(pool)
            .await
            .map_err(|e: sqlx::Error| {
                tracing::error!("Failed to fetch book with id, {}. {:?}", book_id, e);
                match e {
                    sqlx::Error::RowNotFound => BookErrors::BookNotFound(book_id.to_string()),
                    _ => BookErrors::BookQueryError("Error fetching book for update".to_string()),
                }
            })?;

        let title_str = update_body
            .title
            .clone()
            .map(|t| t.as_ref().to_string())
            .unwrap_or(old_book.title);
        let title = ValidStringEntry::parse(title_str).map_err(|e| AppError::ValidationError(e))?;

        let author_str = update_body
            .author
            .clone()
            .map(|t| t.as_ref().to_string())
            .unwrap_or(old_book.author);
        let author =
            ValidStringEntry::parse(author_str).map_err(|e| AppError::ValidationError(e))?;

        let updated_book = BookFormData {
            title,
            author,
            status: update_body.status.clone().unwrap_or(old_book.status),
            category: update_body.category.clone().unwrap_or(old_book.category),
        };

        sqlx::query!(r#"UPDATE books set title = $1, author = $2, status = $3, category = $4, updated_at = now() WHERE id = $5"#,
            updated_book.title.as_ref(),
            updated_book.author.as_ref(),
            updated_book.status.clone() as BookStatus,
            updated_book.category.clone() as BookCategory,
            book_id
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update book: {:?}", e);
            BookErrors::ErrorUpdatingBook(updated_book.title.as_ref().to_string())
        })?;
        Ok(())
    }

    // Get all books
    #[tracing::instrument(name = "get_all_books", skip(pool))]
    pub async fn get_all_books(
        query: BookQueryInfo,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Book>, AppError> {
        let BookQueryInfo {
            category,
            status,
            year_read,
        } = query;

        let books = sqlx::query_as!(
            Book,
            r#"SELECT id, title, author, status AS "status: BookStatus", category AS "category: BookCategory", year_read, created_at, updated_at FROM books ORDER BY year_read"#)
        .fetch_all(pool)
        .await
        .map_err(|e: sqlx::Error| {
            tracing::error!("Failed to fetch books: {:?}", e);
            BookErrors::ErrorFetchingBooks(e.to_string())
        })?;

        let filtered: Vec<Book> = books
            .into_iter()
            .filter(|b| category.as_ref().is_none_or(|c| b.category == *c))
            .filter(|b| status.as_ref().is_none_or(|s| b.status == *s))
            .filter(|b| year_read.is_none_or(|y| b.year_read == y))
            .collect();

        Ok(filtered)
    }

    // #[tracing::instrument(name = get)]
    #[tracing::instrument(name = "add_book",skip(book, pool), fields(book_title = ?book.title, book_author= ?book.author) )]
    pub async fn add_book(book: &BookFormData, pool: &sqlx::PgPool) -> Result<(), AppError> {
        tracing::info!("Adding book to reading list");

        insert_book(book, pool).await?;

        Ok(())
    }

    #[tracing::instrument(name = "delete book", skip(pool))]
    pub async fn delete_book(book_id: Uuid, pool: &sqlx::PgPool) -> Result<(), AppError> {
        let result = sqlx::query!(r#"DELETE FROM books WHERE id = $1"#, book_id)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!(error =? e, "Error deleting book");
                BookErrors::ErrorDeletingBook(e.to_string())
            })?;

        if result.rows_affected() == 0 {
            tracing::warn!(%book_id, "No book found to delete");
            return Err(BookErrors::BookNotFound("No book was deleted".to_string()))?;
        }
        Ok(())
    }
}

#[tracing::instrument(name = "Saving new book detail in database", skip(new_book, pool))]
pub async fn insert_book(new_book: &BookFormData, pool: &sqlx::PgPool) -> Result<(), AppError> {
    sqlx::query!(
        r#"INSERT INTO books (title, author, status, category, year_read) VALUES ($1, $2, $3, $4, $5)"#,
        new_book.title.as_ref(),
        new_book.author.as_ref(),
        new_book.status.clone() as BookStatus,
        new_book.category.clone() as BookCategory,
        Utc::now().year() as i16,
    )
    .execute(pool)
    .await.map_err(|e| {
            tracing::error!("Failed to execute query: {:?}",e);
            BookErrors::ErrorAddingBook(new_book.title.as_ref().to_string())
        })?;
    Ok(())
}
