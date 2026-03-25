use crate::common::api_response::{APIResponse, AppResponse};
use crate::services::books::BookService;
use crate::types::{
    app::AppState,
    books::{Book, BookFormData, BookQueryInfo, UpdateBookFormData},
};
use actix_web::web;
use std::sync::Arc;
use uuid::Uuid;

pub async fn add_book(
    app_state: web::Data<Arc<AppState>>,
    body: web::Json<BookFormData>,
) -> AppResponse<String> {
    BookService::add_book(&body, &app_state.connection).await?;

    Ok(APIResponse::success("Book added successfully".to_string()))
}

pub async fn get_book(
    path: web::Path<Uuid>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<Book> {
    let book = BookService::get_book_by_id(path.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(book))
}

pub async fn get_all_books(
    query: web::Query<BookQueryInfo>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<Vec<Book>> {
    let books = BookService::get_all_books(query.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(books))
}

pub async fn update_book(
    path: web::Path<Uuid>,
    body: web::Json<UpdateBookFormData>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    BookService::update_book(path.into_inner(), &body.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(
        "Book updated successfully".to_string(),
    ))
}

pub async fn delete_book(
    path: web::Path<Uuid>,
    app_state: web::Data<Arc<AppState>>,
) -> AppResponse<String> {
    BookService::delete_book(path.into_inner(), &app_state.connection).await?;

    Ok(APIResponse::success(
        "Book successfully deleted".to_string(),
    ))
}
