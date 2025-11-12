use actix_web::{HttpResponse, get};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    public_key: String,
}

#[get("/v1/transit/wrapping_key")]
pub(super) async fn handle_wrapping_key_get() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        data: ResponseData {
            public_key: "test".to_string(),
        },
    })
}
