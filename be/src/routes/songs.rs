use crate::handlers::songs;
use actix_web::web;

pub fn configure_songs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/songs")
            .route("", web::post().to(songs::add_song))
            .route("", web::get().to(songs::get_all_songs))
            .route("/{id}", web::patch().to(songs::update_song))
            .route("/{id}", web::get().to(songs::get_song_by_id))
            .route("/{id}", web::delete().to(songs::delete_song)),
    );
}
