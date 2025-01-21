use arcozelo_engine::{configuration::get_configuration, startup::run};
use dotenv::dotenv;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    println!("http:://127.0.0.1:{}", port);
    run(listener, connection)?.await
}
