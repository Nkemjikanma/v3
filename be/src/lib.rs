pub mod routes;
use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");

    format!("Hello, {}!", name)
}

async fn books() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn add_book() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn songs() -> HttpResponse {
    HttpResponse::Ok().finish()
}

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
