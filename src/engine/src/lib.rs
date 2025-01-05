use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn donor_events_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn donor_profile_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn login() -> impl Responder {
    HttpResponse::Ok().finish()
}
async fn logout() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allowed_origin("https://localhost:8000")
        //     .allowed_origin_fn(|origin, _req_head| {
        //         origin.as_bytes().ends_with(b".rust-lang.org")
        //     })
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        App::new()
            // .wrap(cors)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/sub", web::post().to(subscribe))
            .route("/login", web::get().to(login))
            .route("/logout", web::post().to(logout))
            .route("/donor-event/action/start-new-event",web::get().to(donor_events_handler))
            .route("/donor-event/action/cancel-new-event",web::get().to(donor_events_handler))
            .route("/donor-profile/action/start-temp-event",web::get().to(donor_profile_handler))
            .route("/donor-profile/action/cancel-temp-event",web::get().to(donor_profile_handler))
            .route("/donor-event",web::get().to(donor_events_handler))
            .route("/donor-profile",web::get().to(donor_profile_handler))
            .route("/1donor-profile/all",web::get().to(donor_profile_handler))
    })
    .listen(listener)?
    .run();

    Ok(server)
}


// #[tokio::main]
// async fn main() {
//     dotenv::dotenv().expect("Failed to load .env file");
//
//     let resp = reqwest::get("https://httpbin.org/ip").await.unwrap();
//
//     let api_address = std::env::var("API_ADDRESS").unwrap_or("127.0.0.1:8000".to_owned());
//     let database_url = std::env::var("DATABASE_URL").expect("Database not found");
//
//     let database_pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await.expect("Failed to connect to database");
//
//     sqlx::migrate!("./migrations").run(&database_pool).await.expect("Failed to run migrations");
//
//     let tcp_listener = tokio::net::TcpListener::bind(api_address).await.unwrap();
//
//     let cors = CorsLayer::new()
//         .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
//         .allow_headers(Any)
//         .allow_methods([Method::GET, Method::POST, Method::OPTIONS]);
//
//     let api = Router::new()
//         .route("/login", post(account_handler::login))
//         .route("/logout", post(account_handler::logout))
//         .route("/donor-event/action/start-new-event", post(donor_events_handler::new_event_started))
//         .route("/donor-event/action/cancel-new-event", post(donor_events_handler::new_event_cancelled))
//         .route("/donor-profile/action/start-temp-profile", post(donor_profile_handler::new_temp_profile_started))
//         .route("/donor-profile/action/cancel-temp-profile", post(donor_profile_handler::new_temp_profile_cancelled))
//         .route("/donor-event", post(donor_events_handler::create_new_event))
//         .route("/donor-profile", post(donor_profile_handler::new_donor))
//         .route("/donor-profile/all", get(donor_profile_handler::get_all_donors))
//         .with_state(database_pool)
//         .layer(cors);
//
//     println!("listening on port 8000");
//     axum::serve(tcp_listener, api).await.unwrap();
// }
//

