use axum::http::HeaderValue;
use axum::routing::post;
use axum::{routing::get, Router};
use endpoints::{account_handler, donor_events_handler, donor_profile_handler};
use hyper::Method;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod endpoints;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_address = std::env::var("API_ADDRESS").unwrap_or("127.0.0.1:8000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("Database not found");

    let database_pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await.expect("Failed to connect to database");

    sqlx::migrate!("./migrations").run(&database_pool).await.expect("Failed to run migrations");

    let tcp_listener = tokio::net::TcpListener::bind(api_address).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS]);

    let api = Router::new()
        .route("/login", post(account_handler::login))
        .route("/logout", post(account_handler::logout))
        .route("/donor-event/action/start-new-event", post(donor_events_handler::new_event_started))
        .route("/donor-event/action/cancel-new-event", post(donor_events_handler::new_event_cancelled))
        .route("/donor-profile/action/start-temp-profile", post(donor_profile_handler::new_temp_profile_started))
        .route("/donor-profile/action/cancel-temp-profile", post(donor_profile_handler::new_temp_profile_cancelled))
        .route("/donor-event", post(donor_events_handler::create_new_event))
        .route("/donor-profile", post(donor_profile_handler::new_donor))
        .route("/donor-profile/all", get(donor_profile_handler::get_all_donors))
        .with_state(database_pool)
        .layer(cors);

    println!("listening on port 8000");
    axum::serve(tcp_listener, api).await.unwrap();
}
