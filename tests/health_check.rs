use std::net::TcpListener;

fn spawn_app() -> String {
    // binding to port 0 tells the OS to find any available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter_api::startup::run(listener).expect("Failed to bind address");

    // Create a tokio spawn instance to run the server in the background so as not to hang the test
    // runner
    let _ = tokio::spawn(server);
    // Return the binded address to the caller
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    // use reqwest to perform HTTP requests for testing
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_with_valid_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=some%20name&email=some_email%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_with_invalid_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=some%20name", "missing email"),
        ("some_email%40gmail.com", "missing name"),
        ("", "missing email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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
