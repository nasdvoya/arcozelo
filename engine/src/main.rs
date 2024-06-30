use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hallo" }))
        .route("/contacts", get(handlers::contacts()))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
