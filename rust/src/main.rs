#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate random_number;


use actix_web::{App, HttpServer};
use dotenv::dotenv;


mod routes;

use actix_web::middleware::Logger;

mod errors;
mod mocks;
mod search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(mocks::api::PartSearchAPI {})
            .service(routes::healthcheck)
            .service(routes::parts)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
