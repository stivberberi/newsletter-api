use reqwest;
use std::net::TcpListener;

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

fn spawn_app() -> String {
    // binding to port 0 tells the OS to find any available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter_api::run(listener).expect("Failed to bind address");

    // Create a tokio spawn instance to run the server in the background so as not to hang the test
    // runner
    let _ = tokio::spawn(server);
    // Return the binded address to the caller
    format!("http://127.0.0.1:{}", port)
}
