use axum::routing::post;
use axum::{routing::get, Form, Router};
use endpoints::card_handler::{self};
use endpoints::{account_handler, donor_events_handler, donor_profile_handler};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

mod endpoints;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_address = std::env::var("API_ADDRESS").unwrap_or("127.0.0.1:8000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("Database not found");

    let database_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let tcp_listener = tokio::net::TcpListener::bind(api_address).await.unwrap();

    let api = Router::new()
        .route("/", get(|| async { "Hallo" }))
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
            get(donor_profile_handler::create_new_temp_profile),
        )
        .with_state(database_pool)
        .layer(CorsLayer::very_permissive());

    println!("listening on port 8000");
    axum::serve(tcp_listener, api).await.unwrap();
}
