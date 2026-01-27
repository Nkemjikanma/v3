use actix_web::HttpResponse;
use actix_web::web;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct BookFormData {
    pub title: String,
    pub category: Option<String>,
}

pub async fn books() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn add_book(form: web::Form<BookFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
