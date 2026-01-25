//! tests/check.rs

use std::net::TcpListener;
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");

    let port = listener.local_addr().unwrap().port();

    let server = be::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn add_book_returns_200() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let body = "title=Anthills%20of%20the%20Savannah&category=leisure";
    let response = client
        .post(&format!("{}/books", &address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn add_book_returns_400() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("title=ant%20hill", "missing category"),
        ("category=technical", "missing name"),
        ("missing", "missng category"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/books", &address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad request when the payload was {}",
            error_msg
        )
    }
}
