use crate::routes::*;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // web::Data will wrap the connection in an Arc so it can be used by the multiple Actix workers
    let db_pool = web::Data::new(db_pool);

    // server takes in a closure because Actix will spawn a new worker process for each core
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
