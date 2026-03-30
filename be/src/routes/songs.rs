use crate::handlers::songs;
use crate::middleware::validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure_songs(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/songs")
            .route("/{id}", web::get().to(songs::get_song_by_id))
            .route("", web::get().to(songs::get_all_songs)),
    )
    .service(
        web::scope("/songs")
            .wrap(auth_middleware)
            .route("", web::post().to(songs::add_song))
            .route("/{id}", web::patch().to(songs::update_song))
            .route("/{id}", web::delete().to(songs::delete_song)),
    );
}
