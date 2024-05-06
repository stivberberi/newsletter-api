use newsletter_api::configuration::get_configuration;
use newsletter_api::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration file.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Running on {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
