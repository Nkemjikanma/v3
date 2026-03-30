use crate::handlers::auth;
use actix_web::web;
pub fn configure_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/login", web::post().to(auth::login)));
}

// pub fn configure_books(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/books")
//             .route("", web::post().to(books::add_book))
//             .route("", web::get().to(books::get_all_books))
//             .route("/{id}", web::get().to(books::get_book))
//             .route("/{id}", web::patch().to(books::update_book))
//             .route("/{id}", web::delete().to(books::delete_book)),
//     );
// }
