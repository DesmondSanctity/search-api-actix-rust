pub mod search;

use actix_web::{web, get, HttpResponse, Responder};
use serde::Deserialize;
use crate::routes::search::search_by_mpn;

#[derive(Debug, Deserialize)]
pub struct Params {
    query: String,
}


#[derive(Debug)]
pub struct CustomError(Option<String>);

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}

//route for parts
#[get("/parts")]
pub async fn parts() -> impl Responder {
    let params = req.query().get("q").unwrap();
    let parts = web::block(|| search_by_mpn(params)).await.unwrap();
    HttpResponse::Ok().body("query successful")
}