use crate::errors::ApiError;
use crate::mocks::api::PartSearchAPI;
use crate::search::highlighting::Highlight;
use actix_web::{web, HttpResponse};
use serde::Serialize;
use serde::Deserialize;
use serde_json::json;
// TODO: You will need add search query parameter and return JSON response with parts with
// highlighted MPNs

#[derive(Serialize, Deserialize)]
pub struct Param {
    pub q: String,
    pub mpn: String
}

pub async fn get_query_result(param: web::Path<Param>, data: web::Data<Highlight>) -> HttpResponse {
    let json_message = json!({});
    HttpResponse::Ok().json(json_message)
}

pub async fn search_by_mpn(
    // Part api you can use for mocking response
    _part_api: web::Data<PartSearchAPI>,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("This doesn't seem right."))
}

#[cfg(test)]
mod tests {
    mod integration {
        use crate::mocks;
        use actix_web::middleware::Logger;
        use actix_web::{test, App};

        #[actix_rt::test]
        async fn test_request() {
            // Here we just add all relevant data to provide with the same objects we would have
            // when running the app when running the app
            let mut app = test::init_service(
                App::new()
                    .wrap(Logger::default())
                    .data(mocks::api::PartSearchAPI {}),
            )
            .await;

            let req = test::TestRequest::get().uri("/").to_request();
            let resp = test::call_service(&mut app, req).await;

            // There are not endpoints configured in this app so we expect the req to fail!
            assert_eq!(resp.status(), 404)
        }
    }
}
