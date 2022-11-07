use crate::errors::ApiError;
use crate::mocks::api::PartSearchAPI;
use crate::mocks::api::SearchAPI;
use actix_web::{web, HttpResponse};

// TODO: You will need add search query parameter and return JSON response with parts with
// highlighted MPNs
pub async fn search_by_mpn(
    _part_api: web::Data<SearchAPI>,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("This doesn't seem right."))
}

pub async fn search_by_mpn_mock(
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
