#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate random_number;

use actix_files::Files;
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
            .default_service(Files::new("/", "../typescript/public/").index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
