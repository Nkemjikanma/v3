//! main.rs
use be::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind IP and Port in main");
    let post = listener.local_addr().unwrap().port();

    run(listener)?.await
}
