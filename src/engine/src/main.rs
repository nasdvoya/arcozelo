use std::net::TcpListener;
use arcozelo_engine::configuration::get_configuration;
use arcozelo_engine::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to get configuration");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("http:://127.0.0.1:{}", port);
    run(listener)?.await
}
