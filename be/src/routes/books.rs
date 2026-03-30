use crate::handlers::books;
use crate::middleware::validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure_books(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/books")
            .route("/{id}", web::get().to(books::get_book))
            .route("", web::get().to(books::get_all_books)),
    )
    .service(
        web::scope("/books")
            .wrap(auth_middleware)
            .route("", web::post().to(books::add_book))
            .route("/{id}", web::patch().to(books::update_book))
            .route("/{id}", web::delete().to(books::delete_book)),
    );
}
