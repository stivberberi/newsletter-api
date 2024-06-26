use newsletter_api::{
    configuration::{get_configuration, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::sync::OnceLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: OnceLock<()> = OnceLock::new();

async fn spawn_app() -> TestApp {
    // ensures we initialise the tracing only once when running tests
    TRACING.get_or_init(|| {
        let default_filter_level = "info".to_string();
        let subscriber_name = "test".to_string();

        // Can't assign `get_subscriber` to a variable based on result of `TEST_LOG` because sink
        // is part of the return type, meaning they can't be cast to the same type.
        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
            init_subscriber(subscriber);
        } else {
            let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
            init_subscriber(subscriber);
        };
    });

    // binding to port 0 tells the OS to find any available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    // randomize database name to create a new instance for each test
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = newsletter_api::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    // Create a tokio spawn instance to run the server in the background so as not to hang the test
    // runner
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate the database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");
    connection_pool
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    // use reqwest to perform HTTP requests for testing
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_with_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=stiv%20berberi&email=stivberberi%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "stivberberi@gmail.com");
    assert_eq!(saved.name, "stiv berberi");
}

#[actix_rt::test]
async fn subscribe_with_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=some%20name", "missing email"),
        ("some_email%40gmail.com", "missing name"),
        ("", "missing email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Add additional logging for failed test cases
            "API did not fail with 400 Bad Request with payload: {}",
            error_message
        );
    }
}
