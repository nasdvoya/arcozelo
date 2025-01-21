use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::PgConnection;
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    dotenv().ok();
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/login", web::get().to(login))
            .route("/logout", web::post().to(logout))
            .route("/donor-event/action/start-new-event", web::get().to(donor_events_handler))
            .route("/donor-event/action/cancel-new-event", web::get().to(donor_events_handler))
            .route("/donor-profile/action/start-temp-event", web::get().to(donor_profile_handler))
            .route("/donor-profile/action/cancel-temp-event", web::get().to(donor_profile_handler))
            .route("/donor-event", web::get().to(donor_events_handler))
            .route("/donor-profile", web::get().to(donor_profile_handler))
            .route("/1donor-profile/all", web::get().to(donor_profile_handler))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
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
