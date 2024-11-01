use axum::routing::post;
use axum::{routing::get, Form, Router};
use endpoints::card_handler::{self};
use endpoints::{account_handler, donor_events_handler, donor_profile_handler};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

mod endpoints;

#[tokio::main]
async fn main() {
    let url = "postgres://postgres:test@localhost/donors";
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await;

    let app = Router::new()
        .route("/s", get(|| async { "Hallo" }))
        .route("/send", post(card_handler::submit_card_handler))
        .route("/login", post(account_handler::login))
        .route("/logout", post(account_handler::logout))
        .route(
            "/user-actions/start-new-event",
            post(donor_events_handler::new_event_started),
        )
        .route(
            "/user-actions/cancel-new-event",
            post(donor_events_handler::new_event_cancelled),
        )
        .route(
            "/create-donor-event",
            post(donor_events_handler::create_new_event),
        )
        .route(
            "/user-actions/start-temp-profile",
            post(donor_profile_handler::new_temp_profile_started),
        )
        .route(
            "/user-actions/cancel-temp-profile",
            post(donor_profile_handler::new_temp_profile_cancelled),
        )
        .route(
            "/create-temp-profile",
            post(donor_profile_handler::create_new_temp_profile),
        )
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
