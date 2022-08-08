//! tests/health_check.rs

// You can inspect what code gets generated using
// `cargo expand --test health-check (<- name of the test file)`
#[tokio::test]
async fn dummy_test() {
    // Arrange
    spawn_app();
    // Bring in reqwest
    let client = reqwest::Client::new();
    
    // Act
    let response = client.get("http://127.0.0.1:8000/health-check")
    .send().await.expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// The only piece that will depend on our application code.
/// If tomorrow we decide to ditch Rust and rewrite our app in Ruby on Rails
/// we can still use the same test suite to check for regressions in our new stack
/// as long as we replace the appropriate trigger in here
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}