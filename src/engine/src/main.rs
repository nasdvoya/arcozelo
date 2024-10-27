use axum::routing::post;
use axum::{routing::get, Form, Router};
use endpoints::card_handler::{self};
use endpoints::donor_events;
use tower_http::cors::CorsLayer;

mod endpoints;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hallo" }))
        .route("/send", post(card_handler::submit_card_handler))
        .route("/donor-event", post(donor_events::donor_event))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
