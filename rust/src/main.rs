#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate random_number;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod errors;
mod mocks;
mod routes;
mod search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(mocks::api::PartSearchAPI {})
            .service(routes::healthcheck)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
