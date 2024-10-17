use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hallo" }))
        .route("/contacts", get(handlers::donors()))
        .route("/test", get(|| async { "Simple value" }))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
