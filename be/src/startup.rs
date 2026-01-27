use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

use crate::routes::{
    books::{add_book, books},
    check::greet,
    songs::songs,
};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/", web::get().to(greet))
                .route("/books", web::get().to(books))
                .route("/books", web::post().to(add_book))
                .route("/songs", web::get().to(songs)),
        )
    })
    // .bind(listener)?
    .listen(listener)?
    .run();

    Ok(server)
}
