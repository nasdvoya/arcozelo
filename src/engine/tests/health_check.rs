//! tests/health_check.rs

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");
    // Assert
    assert!(response.status().is_success());
}

fn spawn_app() {
    let server = arcozelo_engine::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
