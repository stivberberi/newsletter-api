use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .listen(listener)?
        .run();
    Ok(server)
}
