use axum::routing::post;
use axum::{routing::get, Form, Router};
use card_handler::submit_card;
use tower_http::cors::CorsLayer;

mod card_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hallo" }))
        .route("/send", post(submit_card))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
