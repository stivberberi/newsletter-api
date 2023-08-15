use newsletter_api::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set the port to run on
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");
    println!("Running on 127.0.0.1:8000");
    run(listener)?.await
}
