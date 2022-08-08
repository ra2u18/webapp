//! tests/health_check.rs

use std::net::TcpListener;

// You can inspect what code gets generated using
// `cargo expand --test health-check (<- name of the test file)`
#[tokio::test]
async fn dummy_test() {
    // Arrange
    let address = spawn_app();
    // Bring in reqwest
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // Return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
