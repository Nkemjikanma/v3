//! tests/check.rs
use be::config::{Config, DBConfig};
use be::startup::run;
use be::telemetry::{get_subscriber, init_subscriber};
use be::types::{
    app,
    books::{Book, BookCategory, BookFormData, BookStatus},
};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::LazyLock; // Ensures the tracing stack is called initialized once.
use uuid::Uuid;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING); // initialize and lock tracing functions

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // have dynamic db name so we are technically spining up a new db for each test - using
    // maintainance db that ships with postgres for this
    let mut config = Config::load_config().expect("Failed to read config for test");
    config.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&config.database).await;

    let test_app_state = Arc::new(app::AppState {
        app_config: config,
        connection: connection_pool.clone(),
    });

    let server = run(listener, test_app_state).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DBConfig) -> PgPool {
    // create db
    let maintainance_settings = DBConfig {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: SecretString::from("password".to_string()),
        ..config.clone()
    };

    let connection =
        PgConnection::connect(&maintainance_settings.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres")
            .execute(format!(r#"CREATE DATABASE "{}"; "#, config.database_name).as_str())
            .await
            .expect("Failed to create database");

    //migrate db
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the db");

    connection_pool
}

#[tokio::test]
async fn check() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/api", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    println!(
        "heeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrreeeeeeeeeee: {:?}",
        response.status()
    );

    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn add_book_returns_200() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let book: BookFormData = BookFormData {
        title: "Things Fall Apart".to_string(),
        author: "Chinua Achebe".to_string(),
        status: BookStatus::Reading,
        category: BookCategory::Leisure,
    };

    let response = client
        .post(&format!("{}/api/books", &test_app.address))
        .json(&book)
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(r#"SELECT title, author, status as "status: BookStatus", category as "category: BookCategory", year_read FROM books"#,)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.title, "Things Fall Apart");
    assert_eq!(saved.category, BookCategory::Leisure);
}

#[tokio::test]
async fn add_book_returns_400() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let book: BookFormData = BookFormData {
        title: "".to_string(),
        author: "Chinua Achebe".to_string(),
        status: BookStatus::Reading,
        category: BookCategory::Leisure,
    };

    let book2: BookFormData = BookFormData {
        title: "No longer at ease".to_string(),
        author: "".to_string(),
        status: BookStatus::Reading,
        category: BookCategory::Leisure,
    };

    let test_cases: [BookFormData; 2] = [book, book2];

    for item in test_cases.iter() {
        let response = client
            .post(&format!("{}/api/books", &test_app.address))
            .json(item)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad request when the payload was"
        )
    }
}
